use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use std::time::{Duration, Instant};
use sysinfo::{Pid, System};
use tracing::{info, warn};

use crate::config::HealthConfig;

// =============================================================================
// Health Status
// =============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded(String),
    Unhealthy(String),
    Unknown,
}

impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HealthStatus::Healthy => write!(f, "Healthy"),
            HealthStatus::Degraded(msg) => write!(f, "Degraded: {}", msg),
            HealthStatus::Unhealthy(msg) => write!(f, "Unhealthy: {}", msg),
            HealthStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

// =============================================================================
// Health Check Result
// =============================================================================

#[derive(Debug, Clone)]
pub struct HealthCheckResult {
    pub process_name: String,
    pub status: HealthStatus,
    pub level_results: Vec<LevelResult>,
    pub checked_at: Instant,
}

#[derive(Debug, Clone)]
pub struct LevelResult {
    pub level: u8,
    pub name: String,
    pub passed: bool,
    pub message: String,
}

// =============================================================================
// Resource History (Sprint 2.3: sustained monitoring)
// =============================================================================

#[derive(Debug, Clone)]
struct ResourceSample {
    memory_mb: u64,
    cpu_percent: f32,
    timestamp: Instant,
}

#[derive(Debug)]
struct ResourceHistory {
    samples: Vec<ResourceSample>,
    max_samples: usize,
}

impl ResourceHistory {
    fn new(max_samples: usize) -> Self {
        Self {
            samples: Vec::new(),
            max_samples,
        }
    }

    fn add_sample(&mut self, memory_mb: u64, cpu_percent: f32) {
        self.samples.push(ResourceSample {
            memory_mb,
            cpu_percent,
            timestamp: Instant::now(),
        });
        // Keep only recent samples
        if self.samples.len() > self.max_samples {
            self.samples.remove(0);
        }
    }

    /// Check if resource exceeded threshold for sustained period
    fn is_sustained_over(
        &self,
        max_memory_mb: u64,
        max_cpu_percent: f32,
        sustained_secs: u64,
    ) -> (bool, bool) {
        let recent: Vec<&ResourceSample> = if sustained_secs == 0 {
            // When sustained_secs is 0, check all samples
            self.samples.iter().collect()
        } else {
            let cutoff = Instant::now() - Duration::from_secs(sustained_secs);
            self.samples.iter().filter(|s| s.timestamp >= cutoff).collect()
        };

        if recent.len() < 2 {
            return (false, false);
        }

        let memory_exceeded = recent.iter().all(|s| s.memory_mb > max_memory_mb);
        let cpu_exceeded = recent.iter().all(|s| s.cpu_percent > max_cpu_percent);

        (memory_exceeded, cpu_exceeded)
    }

    fn latest(&self) -> Option<&ResourceSample> {
        self.samples.last()
    }
}

// =============================================================================
// Backup Manager (Sprint 2.2: generational backups)
// =============================================================================

pub struct BackupManager;

impl BackupManager {
    const MAX_GENERATIONS: usize = 5;

    /// Create a generational backup of a file
    pub fn create_backup(file_path: &str) -> std::io::Result<String> {
        let path = Path::new(file_path);
        if !path.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Source file not found",
            ));
        }

        // Shift existing backups: .backup.4 -> .backup.5, .backup.3 -> .backup.4, etc.
        for i in (1..Self::MAX_GENERATIONS).rev() {
            let old = format!("{}.backup.{}", file_path, i);
            let new = format!("{}.backup.{}", file_path, i + 1);
            if Path::new(&old).exists() {
                let _ = std::fs::rename(&old, &new);
            }
        }

        // Delete oldest backup if it exceeds max generations
        let oldest = format!("{}.backup.{}", file_path, Self::MAX_GENERATIONS);
        if Path::new(&oldest).exists() {
            let _ = std::fs::remove_file(&oldest);
        }

        // Copy current file to .backup.1
        let backup_path = format!("{}.backup.1", file_path);
        std::fs::copy(file_path, &backup_path)?;

        info!("Created backup: {}", backup_path);
        Ok(backup_path)
    }

    /// Restore the most recent backup
    pub fn restore_latest(file_path: &str) -> std::io::Result<()> {
        // Find the most recent backup
        for i in 1..=Self::MAX_GENERATIONS {
            let backup = format!("{}.backup.{}", file_path, i);
            if Path::new(&backup).exists() {
                std::fs::copy(&backup, file_path)?;
                info!("Restored backup: {} -> {}", backup, file_path);
                return Ok(());
            }
        }

        // Fallback: try legacy .backup file
        let legacy = format!("{}.backup", file_path);
        if Path::new(&legacy).exists() {
            std::fs::copy(&legacy, file_path)?;
            info!("Restored legacy backup: {} -> {}", legacy, file_path);
            return Ok(());
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No backup found to restore",
        ))
    }

    /// List available backups
    pub fn list_backups(file_path: &str) -> Vec<String> {
        let mut backups = Vec::new();
        for i in 1..=Self::MAX_GENERATIONS {
            let backup = format!("{}.backup.{}", file_path, i);
            if Path::new(&backup).exists() {
                backups.push(backup);
            }
        }
        backups
    }
}

// =============================================================================
// Log Pattern Watcher (Sprint 2.1: advanced pattern matching)
// =============================================================================

#[derive(Debug)]
struct LogPatternState {
    last_size: u64,
    error_count: u32,
    warning_count: u32,
    last_error: Option<String>,
}

impl LogPatternState {
    fn new() -> Self {
        Self {
            last_size: 0,
            error_count: 0,
            warning_count: 0,
            last_error: None,
        }
    }
}

// =============================================================================
// Health Checker
// =============================================================================

pub struct HealthChecker {
    system: System,
    last_check: HashMap<String, HealthCheckResult>,
    resource_history: HashMap<String, ResourceHistory>,
    log_states: HashMap<String, LogPatternState>,
    http_consecutive_failures: HashMap<String, u32>,
}

impl HealthChecker {
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
            last_check: HashMap::new(),
            resource_history: HashMap::new(),
            log_states: HashMap::new(),
            http_consecutive_failures: HashMap::new(),
        }
    }

    /// Run all health checks for a process
    pub async fn check_process(
        &mut self,
        name: &str,
        pid: Option<u32>,
        health_config: &HealthConfig,
    ) -> HealthCheckResult {
        let mut level_results = Vec::new();

        // Level 1: Process alive check
        if health_config.process_alive {
            let result = self.check_process_alive(pid);
            level_results.push(result);
        }

        // Level 2: Log activity + pattern matching
        if let Some(ref log_file) = health_config.log_file {
            let result = self
                .check_log_activity_and_patterns(
                    name,
                    log_file,
                    health_config.log_activity_timeout,
                    health_config.log_pattern.as_deref(),
                )
                .await;
            level_results.push(result);
        }

        // Level 3: Config file validation with generational backup
        if let Some(ref config_file) = health_config.config_file {
            if health_config.validate_json {
                let result =
                    self.check_json_config_with_backup(config_file, health_config.auto_backup);
                level_results.push(result);
            }
        }

        // Level 4: Sustained resource usage check
        if let Some(pid_val) = pid {
            let result = self.check_resources_sustained(
                name,
                pid_val,
                health_config.max_memory_mb,
                health_config.max_cpu_percent,
                health_config.check_interval,
            );
            level_results.push(result);
        }

        // Level 5: HTTP endpoint with retry tracking
        if let Some(ref endpoint) = health_config.http_endpoint {
            let result = self
                .check_http_endpoint_with_retries(name, endpoint, health_config.http_timeout)
                .await;
            level_results.push(result);
        }

        // Determine overall status
        let status = Self::aggregate_status(&level_results);

        let check_result = HealthCheckResult {
            process_name: name.to_string(),
            status,
            level_results,
            checked_at: Instant::now(),
        };

        self.last_check
            .insert(name.to_string(), check_result.clone());
        check_result
    }

    // =========================================================================
    // Level 1: Process Alive
    // =========================================================================

    fn check_process_alive(&mut self, pid: Option<u32>) -> LevelResult {
        match pid {
            Some(pid_val) => {
                self.system.refresh_processes();
                let alive = self.system.process(Pid::from_u32(pid_val)).is_some();

                LevelResult {
                    level: 1,
                    name: "Process Alive".to_string(),
                    passed: alive,
                    message: if alive {
                        format!("PID {} is running", pid_val)
                    } else {
                        format!("PID {} is not found", pid_val)
                    },
                }
            }
            None => LevelResult {
                level: 1,
                name: "Process Alive".to_string(),
                passed: false,
                message: "No PID available".to_string(),
            },
        }
    }

    // =========================================================================
    // Level 2: Log Activity + Pattern Matching (Sprint 2.1)
    // =========================================================================

    async fn check_log_activity_and_patterns(
        &mut self,
        name: &str,
        log_file: &str,
        timeout_secs: u64,
        error_pattern: Option<&str>,
    ) -> LevelResult {
        let path = Path::new(log_file);

        if !path.exists() {
            return LevelResult {
                level: 2,
                name: "Log Activity".to_string(),
                passed: true,
                message: format!("Log file not found: {}", log_file),
            };
        }

        // Check log modification time (activity)
        let activity_result = match std::fs::metadata(path) {
            Ok(metadata) => {
                let modified = metadata
                    .modified()
                    .map(|t| t.elapsed().unwrap_or(Duration::from_secs(0)))
                    .unwrap_or(Duration::from_secs(u64::MAX));

                let timeout = Duration::from_secs(timeout_secs);
                let is_active = modified < timeout;

                (is_active, modified.as_secs())
            }
            Err(_) => (false, u64::MAX),
        };

        // Check for error patterns in new log content (Sprint 2.1.3)
        let pattern_result = if let Some(pattern_str) = error_pattern {
            self.scan_log_for_patterns(name, log_file, pattern_str)
        } else {
            None
        };

        let (is_active, elapsed_secs) = activity_result;

        // Build result combining activity and pattern checks
        if !is_active {
            LevelResult {
                level: 2,
                name: "Log Activity".to_string(),
                passed: false,
                message: format!(
                    "Log stale: last updated {}s ago (timeout: {}s)",
                    elapsed_secs, timeout_secs
                ),
            }
        } else if let Some(error_msg) = pattern_result {
            LevelResult {
                level: 2,
                name: "Log Activity".to_string(),
                passed: false,
                message: format!("Log active but errors detected: {}", error_msg),
            }
        } else {
            LevelResult {
                level: 2,
                name: "Log Activity".to_string(),
                passed: true,
                message: format!("Log updated {}s ago", elapsed_secs),
            }
        }
    }

    /// Scan log file for error patterns, only reading new content since last check
    fn scan_log_for_patterns(
        &mut self,
        name: &str,
        log_file: &str,
        pattern_str: &str,
    ) -> Option<String> {
        let state = self
            .log_states
            .entry(name.to_string())
            .or_insert_with(LogPatternState::new);

        let metadata = std::fs::metadata(log_file).ok()?;
        let current_size = metadata.len();

        if current_size <= state.last_size {
            state.last_size = current_size;
            return None;
        }

        // Read only new content (use floor_char_boundary to avoid UTF-8 panic)
        let content = std::fs::read_to_string(log_file).ok()?;
        let offset = state.last_size as usize;
        let new_content = if state.last_size > 0 && offset < content.len() {
            // Find a valid UTF-8 char boundary at or after the offset
            let safe_offset = if content.is_char_boundary(offset) {
                offset
            } else {
                // Search forward for a valid char boundary (max 4 bytes for UTF-8)
                (offset..content.len())
                    .find(|&i| content.is_char_boundary(i))
                    .unwrap_or(content.len())
            };
            &content[safe_offset..]
        } else {
            // First scan or file was rotated - skip full scan
            state.last_size = current_size;
            return None;
        };

        state.last_size = current_size;

        // Match error patterns
        if let Ok(re) = Regex::new(pattern_str) {
            let mut errors = Vec::new();
            for line in new_content.lines() {
                if re.is_match(line) {
                    state.error_count += 1;
                    state.last_error = Some(line.to_string());
                    errors.push(line.trim().to_string());
                }
            }

            if !errors.is_empty() {
                let count = errors.len();
                let last = errors.last().unwrap().clone();
                let truncated = if last.len() > 100 {
                    format!("{}...", &last[..100])
                } else {
                    last
                };
                return Some(format!("{} errors found, latest: {}", count, truncated));
            }
        }

        None
    }

    // =========================================================================
    // Level 3: JSON Config Validation with Generational Backup (Sprint 2.2)
    // =========================================================================

    fn check_json_config_with_backup(
        &self,
        config_file: &str,
        auto_backup: bool,
    ) -> LevelResult {
        let path = Path::new(config_file);

        if !path.exists() {
            return LevelResult {
                level: 3,
                name: "Config Validation".to_string(),
                passed: true,
                message: format!("Config file not found: {}", config_file),
            };
        }

        match std::fs::read_to_string(path) {
            Ok(content) => match serde_json::from_str::<serde_json::Value>(&content) {
                Ok(_) => {
                    // Valid JSON - create generational backup if enabled
                    // Only create backup if content changed since last backup (prevents spam)
                    if auto_backup {
                        let backup_path = format!("{}.backup.1", config_file);
                        let should_backup = if Path::new(&backup_path).exists() {
                            std::fs::read_to_string(&backup_path).ok()
                                != Some(content.clone())
                        } else {
                            true
                        };

                        if should_backup {
                            if let Err(e) = BackupManager::create_backup(config_file) {
                                warn!("Failed to create backup for {}: {}", config_file, e);
                            }
                        }
                    }

                    LevelResult {
                        level: 3,
                        name: "Config Validation".to_string(),
                        passed: true,
                        message: "JSON config is valid".to_string(),
                    }
                }
                Err(e) => {
                    warn!("JSON validation failed for {}: {}", config_file, e);

                    // Auto-rollback: restore from backup (Sprint 2.2.3)
                    if auto_backup {
                        match BackupManager::restore_latest(config_file) {
                            Ok(()) => {
                                info!("Auto-restored config from backup: {}", config_file);
                            }
                            Err(re) => {
                                warn!("Failed to restore backup: {}", re);
                            }
                        }
                    }

                    LevelResult {
                        level: 3,
                        name: "Config Validation".to_string(),
                        passed: false,
                        message: format!("Invalid JSON: {} (rollback attempted)", e),
                    }
                }
            },
            Err(e) => LevelResult {
                level: 3,
                name: "Config Validation".to_string(),
                passed: false,
                message: format!("Failed to read config: {}", e),
            },
        }
    }

    // =========================================================================
    // Level 4: Sustained Resource Monitoring (Sprint 2.3)
    // =========================================================================

    fn check_resources_sustained(
        &mut self,
        name: &str,
        pid: u32,
        max_memory_mb: u64,
        max_cpu_percent: f32,
        check_interval: u64,
    ) -> LevelResult {
        self.system.refresh_processes();

        match self.system.process(Pid::from_u32(pid)) {
            Some(process) => {
                let memory_mb = process.memory() / (1024 * 1024);
                let cpu_percent = process.cpu_usage();

                // Record sample in history
                let history = self
                    .resource_history
                    .entry(name.to_string())
                    .or_insert_with(|| ResourceHistory::new(60)); // Keep 60 samples (~30 min at 30s intervals)

                history.add_sample(memory_mb, cpu_percent);

                // Check sustained threshold (over check_interval period)
                let (memory_sustained, cpu_sustained) =
                    history.is_sustained_over(max_memory_mb, max_cpu_percent, check_interval * 3);

                let memory_ok = !memory_sustained;
                let cpu_ok = !cpu_sustained;

                // Also check instant threshold (immediate alert for extreme values)
                let memory_critical = memory_mb > max_memory_mb * 2;
                let cpu_critical = cpu_percent > 99.0;

                let passed = memory_ok && cpu_ok && !memory_critical && !cpu_critical;

                let mut msg = format!(
                    "Memory: {}MB/{}MB, CPU: {:.1}%/{:.1}%",
                    memory_mb, max_memory_mb, cpu_percent, max_cpu_percent
                );

                if memory_sustained {
                    msg.push_str(" [SUSTAINED memory exceeded]");
                }
                if cpu_sustained {
                    msg.push_str(" [SUSTAINED CPU exceeded]");
                }
                if memory_critical {
                    msg.push_str(" [CRITICAL: 2x memory limit]");
                }

                LevelResult {
                    level: 4,
                    name: "Resource Usage".to_string(),
                    passed,
                    message: msg,
                }
            }
            None => LevelResult {
                level: 4,
                name: "Resource Usage".to_string(),
                passed: false,
                message: format!("Process {} not found", pid),
            },
        }
    }

    // =========================================================================
    // Level 5: HTTP Endpoint with Retry Tracking (Sprint 2.4)
    // =========================================================================

    async fn check_http_endpoint_with_retries(
        &mut self,
        name: &str,
        endpoint: &str,
        timeout_secs: u64,
    ) -> LevelResult {
        let url = endpoint
            .trim_start_matches("http://")
            .trim_start_matches("https://");
        let addr = if url.contains('/') {
            url.split('/').next().unwrap_or(url)
        } else {
            url
        };

        let timeout = Duration::from_secs(timeout_secs);

        // Try up to 2 times with a short delay
        let mut last_err = String::new();
        for attempt in 0..2 {
            if attempt > 0 {
                tokio::time::sleep(Duration::from_millis(500)).await;
            }

            match tokio::time::timeout(timeout, tokio::net::TcpStream::connect(addr)).await {
                Ok(Ok(_stream)) => {
                    // Reset failure counter on success
                    self.http_consecutive_failures.remove(name);

                    return LevelResult {
                        level: 5,
                        name: "HTTP Endpoint".to_string(),
                        passed: true,
                        message: format!("Endpoint {} is reachable", endpoint),
                    };
                }
                Ok(Err(e)) => {
                    last_err = format!("{}", e);
                }
                Err(_) => {
                    last_err = format!("timed out after {}s", timeout_secs);
                }
            }
        }

        // Track consecutive failures
        let failures = self
            .http_consecutive_failures
            .entry(name.to_string())
            .or_insert(0);
        *failures += 1;

        let consecutive = *failures;

        LevelResult {
            level: 5,
            name: "HTTP Endpoint".to_string(),
            passed: false,
            message: format!(
                "Endpoint {} unreachable: {} (consecutive failures: {})",
                endpoint, last_err, consecutive
            ),
        }
    }

    // =========================================================================
    // Status Aggregation
    // =========================================================================

    fn aggregate_status(results: &[LevelResult]) -> HealthStatus {
        if results.is_empty() {
            return HealthStatus::Unknown;
        }

        let failed: Vec<&LevelResult> = results.iter().filter(|r| !r.passed).collect();

        if failed.is_empty() {
            HealthStatus::Healthy
        } else {
            // Level 1 failure (process dead) is always critical
            let has_critical = failed.iter().any(|r| r.level == 1);
            if has_critical {
                HealthStatus::Unhealthy(failed[0].message.clone())
            } else if failed.len() == 1 {
                HealthStatus::Degraded(failed[0].message.clone())
            } else {
                let msgs: Vec<&str> = failed.iter().map(|r| r.message.as_str()).collect();
                HealthStatus::Unhealthy(msgs.join("; "))
            }
        }
    }

    /// Get the last health check result for a process
    pub fn last_result(&self, name: &str) -> Option<&HealthCheckResult> {
        self.last_check.get(name)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_health_status_display() {
        assert_eq!(HealthStatus::Healthy.to_string(), "Healthy");
        assert_eq!(
            HealthStatus::Degraded("test".to_string()).to_string(),
            "Degraded: test"
        );
        assert_eq!(
            HealthStatus::Unhealthy("critical".to_string()).to_string(),
            "Unhealthy: critical"
        );
    }

    #[test]
    fn test_aggregate_all_passed() {
        let results = vec![
            LevelResult {
                level: 1,
                name: "Test".to_string(),
                passed: true,
                message: "OK".to_string(),
            },
            LevelResult {
                level: 2,
                name: "Test2".to_string(),
                passed: true,
                message: "OK".to_string(),
            },
        ];

        assert_eq!(
            HealthChecker::aggregate_status(&results),
            HealthStatus::Healthy
        );
    }

    #[test]
    fn test_aggregate_critical_failure() {
        let results = vec![LevelResult {
            level: 1,
            name: "Process Alive".to_string(),
            passed: false,
            message: "PID not found".to_string(),
        }];

        match HealthChecker::aggregate_status(&results) {
            HealthStatus::Unhealthy(_) => {}
            other => panic!("Expected Unhealthy, got {:?}", other),
        }
    }

    #[test]
    fn test_aggregate_degraded() {
        let results = vec![
            LevelResult {
                level: 1,
                name: "Process Alive".to_string(),
                passed: true,
                message: "OK".to_string(),
            },
            LevelResult {
                level: 2,
                name: "Log Activity".to_string(),
                passed: false,
                message: "Log stale".to_string(),
            },
        ];

        match HealthChecker::aggregate_status(&results) {
            HealthStatus::Degraded(_) => {}
            other => panic!("Expected Degraded, got {:?}", other),
        }
    }

    #[test]
    fn test_aggregate_empty_is_unknown() {
        assert_eq!(
            HealthChecker::aggregate_status(&[]),
            HealthStatus::Unknown
        );
    }

    #[test]
    fn test_aggregate_multi_failure_is_unhealthy() {
        let results = vec![
            LevelResult {
                level: 2,
                name: "Log".to_string(),
                passed: false,
                message: "stale".to_string(),
            },
            LevelResult {
                level: 4,
                name: "Resources".to_string(),
                passed: false,
                message: "memory high".to_string(),
            },
        ];

        match HealthChecker::aggregate_status(&results) {
            HealthStatus::Unhealthy(msg) => {
                assert!(msg.contains("stale"));
                assert!(msg.contains("memory high"));
            }
            other => panic!("Expected Unhealthy, got {:?}", other),
        }
    }

    // Sprint 2.2: Backup manager tests
    #[test]
    fn test_backup_create_and_restore() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test.json");
        let file_str = file_path.to_str().unwrap();

        // Create initial file
        std::fs::write(&file_path, r#"{"key": "value1"}"#).unwrap();

        // Create backup
        let backup = BackupManager::create_backup(file_str).unwrap();
        assert!(Path::new(&backup).exists());

        // Modify original
        std::fs::write(&file_path, r#"{"key": "value2"}"#).unwrap();

        // Create second backup
        BackupManager::create_backup(file_str).unwrap();

        // Verify we have 2 backups
        let backups = BackupManager::list_backups(file_str);
        assert_eq!(backups.len(), 2);

        // Restore latest
        BackupManager::restore_latest(file_str).unwrap();
        let content = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, r#"{"key": "value2"}"#);
    }

    #[test]
    fn test_backup_max_generations() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test.json");
        let file_str = file_path.to_str().unwrap();

        // Create file and 6 backups (max is 5)
        for i in 0..6 {
            std::fs::write(&file_path, format!(r#"{{"v":{}}}"#, i)).unwrap();
            BackupManager::create_backup(file_str).unwrap();
        }

        let backups = BackupManager::list_backups(file_str);
        assert!(backups.len() <= BackupManager::MAX_GENERATIONS);
    }

    // Sprint 2.3: Resource history tests
    #[test]
    fn test_resource_history_sustained() {
        let mut history = ResourceHistory::new(10);

        // Add samples over threshold
        for _ in 0..5 {
            history.add_sample(3000, 95.0);
        }

        let (mem_exceeded, cpu_exceeded) = history.is_sustained_over(2048, 90.0, 0);
        assert!(mem_exceeded);
        assert!(cpu_exceeded);
    }

    #[test]
    fn test_resource_history_not_sustained() {
        let mut history = ResourceHistory::new(10);

        // Mix of over and under threshold
        history.add_sample(1000, 50.0);
        history.add_sample(3000, 95.0);
        history.add_sample(1000, 50.0);

        let (mem_exceeded, _) = history.is_sustained_over(2048, 90.0, 0);
        assert!(!mem_exceeded);
    }

    #[test]
    fn test_resource_history_max_samples() {
        let mut history = ResourceHistory::new(3);

        for i in 0..5 {
            history.add_sample(i * 100, i as f32 * 10.0);
        }

        assert_eq!(history.samples.len(), 3);
    }

    // Sprint 2.1: Log pattern tests
    #[test]
    fn test_log_pattern_detection() {
        let dir = tempfile::tempdir().unwrap();
        let log_path = dir.path().join("test.log");
        let log_str = log_path.to_str().unwrap();

        // Write initial content
        std::fs::write(&log_path, "INFO: starting up\n").unwrap();

        let mut checker = HealthChecker::new();

        // First scan sets baseline
        let result = checker.scan_log_for_patterns("test", log_str, "ERROR|FATAL");
        assert!(result.is_none());

        // Append error content
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(&log_path)
            .unwrap();
        writeln!(file, "ERROR: something went wrong").unwrap();
        writeln!(file, "INFO: continuing").unwrap();
        writeln!(file, "FATAL: critical failure").unwrap();

        // Second scan should detect errors
        let result = checker.scan_log_for_patterns("test", log_str, "ERROR|FATAL");
        assert!(result.is_some());
        let msg = result.unwrap();
        assert!(msg.contains("2 errors found"));
    }

    // Sprint 2.4: HTTP consecutive failures
    #[test]
    fn test_http_failure_tracking() {
        let mut checker = HealthChecker::new();

        // Simulate failures
        checker
            .http_consecutive_failures
            .insert("test".to_string(), 3);
        assert_eq!(checker.http_consecutive_failures["test"], 3);

        // Simulate reset on success
        checker.http_consecutive_failures.remove("test");
        assert!(!checker.http_consecutive_failures.contains_key("test"));
    }
}
