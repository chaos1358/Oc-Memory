use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

// =============================================================================
// Top-level Configuration
// =============================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GuardianConfig {
    pub processes: HashMap<String, ProcessConfig>,
    #[serde(default)]
    pub recovery: RecoveryConfig,
    #[serde(default)]
    pub logging: LoggingConfig,
    #[serde(default)]
    pub memory: MemoryConfig,
    #[serde(default)]
    pub notifications: NotificationConfig,
    #[serde(default)]
    pub macos: MacOsConfig,
    #[serde(default)]
    pub advanced: AdvancedConfig,
}

// =============================================================================
// Process Configuration
// =============================================================================

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProcessConfig {
    pub name: String,
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default = "default_working_dir")]
    pub working_dir: String,
    #[serde(default)]
    pub env: HashMap<String, String>,
    #[serde(default)]
    pub depends_on: Vec<String>,
    #[serde(default = "default_true")]
    pub wait_for_dependency: bool,
    #[serde(default = "default_true")]
    pub auto_restart: bool,
    #[serde(default = "default_restart_delay")]
    pub restart_delay: u64,
    #[serde(default)]
    pub health: HealthConfig,
    #[serde(default)]
    pub ready: ReadyConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HealthConfig {
    #[serde(default = "default_true")]
    pub process_alive: bool,
    #[serde(default)]
    pub log_file: Option<String>,
    #[serde(default = "default_log_activity_timeout")]
    pub log_activity_timeout: u64,
    #[serde(default)]
    pub log_pattern: Option<String>,
    #[serde(default)]
    pub config_file: Option<String>,
    #[serde(default)]
    pub validate_json: bool,
    #[serde(default)]
    pub auto_backup: bool,
    #[serde(default = "default_max_memory_mb")]
    pub max_memory_mb: u64,
    #[serde(default = "default_max_cpu_percent")]
    pub max_cpu_percent: f32,
    #[serde(default = "default_check_interval")]
    pub check_interval: u64,
    #[serde(default)]
    pub http_endpoint: Option<String>,
    #[serde(default = "default_http_timeout")]
    pub http_timeout: u64,
    #[serde(default = "default_http_interval")]
    pub http_interval: u64,
}

impl Default for HealthConfig {
    fn default() -> Self {
        Self {
            process_alive: true,
            log_file: None,
            log_activity_timeout: default_log_activity_timeout(),
            log_pattern: None,
            config_file: None,
            validate_json: false,
            auto_backup: false,
            max_memory_mb: default_max_memory_mb(),
            max_cpu_percent: default_max_cpu_percent(),
            check_interval: default_check_interval(),
            http_endpoint: None,
            http_timeout: default_http_timeout(),
            http_interval: default_http_interval(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ReadyConfig {
    #[serde(default = "default_ready_method")]
    pub method: ReadyMethod,
    #[serde(default)]
    pub pattern: Option<String>,
    #[serde(default)]
    pub port: Option<u16>,
    #[serde(default = "default_ready_timeout")]
    pub timeout: u64,
}

impl Default for ReadyConfig {
    fn default() -> Self {
        Self {
            method: ReadyMethod::Time,
            pattern: None,
            port: None,
            timeout: default_ready_timeout(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ReadyMethod {
    Log,
    Port,
    Time,
}

// =============================================================================
// Recovery Configuration
// =============================================================================

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RecoveryConfig {
    #[serde(default = "default_max_restarts")]
    pub max_restarts: u32,
    #[serde(default = "default_restart_window")]
    pub restart_window: u64,
    #[serde(default = "default_backoff_strategy")]
    pub backoff_strategy: String,
    #[serde(default = "default_initial_backoff")]
    pub initial_backoff: u64,
    #[serde(default = "default_max_backoff")]
    pub max_backoff: u64,
    #[serde(default = "default_give_up_action")]
    pub give_up_action: String,
    #[serde(default)]
    pub scenarios: Vec<RecoveryScenario>,
}

impl Default for RecoveryConfig {
    fn default() -> Self {
        Self {
            max_restarts: default_max_restarts(),
            restart_window: default_restart_window(),
            backoff_strategy: default_backoff_strategy(),
            initial_backoff: default_initial_backoff(),
            max_backoff: default_max_backoff(),
            give_up_action: default_give_up_action(),
            scenarios: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RecoveryScenario {
    pub name: String,
    pub trigger: String,
    pub action: String,
    #[serde(default)]
    pub backup_path: Option<String>,
    #[serde(default)]
    pub then: Option<String>,
    #[serde(default)]
    pub cascade: bool,
    #[serde(default = "default_grace_period")]
    pub grace_period: u64,
    #[serde(default)]
    pub max_backoff: Option<u64>,
    #[serde(default)]
    pub notify: bool,
}

// =============================================================================
// Logging Configuration
// =============================================================================

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoggingConfig {
    #[serde(default = "default_log_output")]
    pub output: String,
    #[serde(default = "default_log_level")]
    pub level: String,
    #[serde(default = "default_rotation")]
    pub rotation: String,
    #[serde(default = "default_log_max_size")]
    pub max_size: String,
    #[serde(default = "default_log_max_files")]
    pub max_files: u32,
    #[serde(default = "default_log_format")]
    pub format: String,
    #[serde(default = "default_timestamp_format")]
    pub timestamp_format: String,
    #[serde(default = "default_true")]
    pub capture_stdout: bool,
    #[serde(default = "default_true")]
    pub capture_stderr: bool,
    #[serde(default)]
    pub managed_logs: ManagedLogs,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            output: default_log_output(),
            level: default_log_level(),
            rotation: default_rotation(),
            max_size: default_log_max_size(),
            max_files: default_log_max_files(),
            format: default_log_format(),
            timestamp_format: default_timestamp_format(),
            capture_stdout: true,
            capture_stderr: true,
            managed_logs: ManagedLogs::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ManagedLogs {
    #[serde(default)]
    pub files: Vec<ManagedLogFile>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ManagedLogFile {
    pub path: String,
    #[serde(default = "default_rotation")]
    pub rotation: String,
    #[serde(default = "default_log_max_size")]
    pub max_size: String,
    #[serde(default = "default_log_max_files")]
    pub max_files: u32,
}

// =============================================================================
// Memory Compression Configuration
// =============================================================================

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MemoryConfig {
    #[serde(default)]
    pub compression: CompressionConfig,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            compression: CompressionConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CompressionConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_token_threshold")]
    pub token_threshold: u64,
    #[serde(default)]
    pub schedule_enabled: bool,
    #[serde(default = "default_schedule")]
    pub schedule: String,
    #[serde(default = "default_schedule_time")]
    pub schedule_time: String,
    #[serde(default = "default_compression_target")]
    pub compression_target: f64,
    #[serde(default = "default_min_compression_ratio")]
    pub min_compression_ratio: f64,
    #[serde(default)]
    pub target_files: Vec<String>,
    #[serde(default = "default_compression_method")]
    pub method: String,
    #[serde(default)]
    pub llm_model: Option<String>,
    #[serde(default)]
    pub llm_api_key_env: Option<String>,
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,
    #[serde(default = "default_retry_delay")]
    pub retry_delay: u64,
    #[serde(default = "default_true")]
    pub log_compression_stats: bool,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            token_threshold: default_token_threshold(),
            schedule_enabled: false,
            schedule: default_schedule(),
            schedule_time: default_schedule_time(),
            compression_target: default_compression_target(),
            min_compression_ratio: default_min_compression_ratio(),
            target_files: Vec::new(),
            method: default_compression_method(),
            llm_model: None,
            llm_api_key_env: None,
            max_retries: default_max_retries(),
            retry_delay: default_retry_delay(),
            log_compression_stats: true,
        }
    }
}

// =============================================================================
// Notification Configuration
// =============================================================================

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NotificationConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub email: Option<EmailConfig>,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            email: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EmailConfig {
    pub smtp_server: String,
    #[serde(default = "default_smtp_port")]
    pub smtp_port: u16,
    #[serde(default = "default_true")]
    pub smtp_tls: bool,
    #[serde(default)]
    pub smtp_user: Option<String>,
    #[serde(default)]
    pub smtp_password_env: Option<String>,
    pub from: String,
    pub to: Vec<String>,
    #[serde(default)]
    pub events: Vec<String>,
    #[serde(default)]
    pub template: Option<EmailTemplate>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EmailTemplate {
    #[serde(default)]
    pub subject: Option<String>,
    #[serde(default)]
    pub body: Option<String>,
}

// =============================================================================
// macOS Configuration
// =============================================================================

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MacOsConfig {
    #[serde(default)]
    pub prevent_sleep: bool,
    #[serde(default = "default_true")]
    pub use_caffeinate: bool,
    #[serde(default = "default_true")]
    pub restore_sleep_on_exit: bool,
}

impl Default for MacOsConfig {
    fn default() -> Self {
        Self {
            prevent_sleep: false,
            use_caffeinate: false, // pmset is the only reliable method for lid-close prevention
            restore_sleep_on_exit: true,
        }
    }
}

// =============================================================================
// Advanced Configuration
// =============================================================================

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AdvancedConfig {
    #[serde(default = "default_supervisor_interval")]
    pub supervisor_interval: u64,
    #[serde(default = "default_spawn_timeout")]
    pub spawn_timeout: u64,
    #[serde(default = "default_shutdown_grace_period")]
    pub shutdown_grace_period: u64,
    #[serde(default = "default_pid_file")]
    pub pid_file: String,
}

impl Default for AdvancedConfig {
    fn default() -> Self {
        Self {
            supervisor_interval: default_supervisor_interval(),
            spawn_timeout: default_spawn_timeout(),
            shutdown_grace_period: default_shutdown_grace_period(),
            pid_file: default_pid_file(),
        }
    }
}

// =============================================================================
// Config Loading & Validation
// =============================================================================

/// Load and validate configuration from a TOML file
pub fn load_config(path: &Path) -> Result<GuardianConfig> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path.display()))?;

    let mut config: GuardianConfig =
        toml::from_str(&content).with_context(|| "Failed to parse TOML config")?;

    // Expand ~ in paths
    expand_paths(&mut config);

    // Validate configuration
    validate_config(&config)?;

    Ok(config)
}

/// Expand ~ to home directory in all path fields
fn expand_paths(config: &mut GuardianConfig) {
    let home = dirs_home();

    for process in config.processes.values_mut() {
        process.working_dir = expand_tilde(&process.working_dir, &home);

        if let Some(ref mut log_file) = process.health.log_file {
            *log_file = expand_tilde(log_file, &home);
        }
        if let Some(ref mut config_file) = process.health.config_file {
            *config_file = expand_tilde(config_file, &home);
        }
    }

    for scenario in &mut config.recovery.scenarios {
        if let Some(ref mut backup_path) = scenario.backup_path {
            *backup_path = expand_tilde(backup_path, &home);
        }
    }

    for managed in &mut config.logging.managed_logs.files {
        managed.path = expand_tilde(&managed.path, &home);
    }

    for target in &mut config.memory.compression.target_files {
        *target = expand_tilde(target, &home);
    }
}

fn expand_tilde(path: &str, home: &str) -> String {
    if path.starts_with("~/") || path.starts_with("~\\") {
        format!("{}{}", home, &path[1..])
    } else {
        path.to_string()
    }
}

fn dirs_home() -> String {
    #[cfg(target_os = "windows")]
    {
        std::env::var("USERPROFILE").unwrap_or_else(|_| "C:\\Users\\default".to_string())
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::env::var("HOME").unwrap_or_else(|_| "/home/default".to_string())
    }
}

/// Validate configuration integrity
fn validate_config(config: &GuardianConfig) -> Result<()> {
    // Check that at least one process is defined
    if config.processes.is_empty() {
        anyhow::bail!("No processes defined in configuration");
    }

    // Validate process names and dependencies
    for (name, process) in &config.processes {
        // Validate dependencies exist
        for dep in &process.depends_on {
            if !config.processes.contains_key(dep) {
                anyhow::bail!(
                    "Process '{}' depends on '{}', which is not defined",
                    name,
                    dep
                );
            }
        }

        // Validate command is not empty
        if process.command.is_empty() {
            anyhow::bail!("Process '{}' has an empty command", name);
        }
    }

    // Check for cyclic dependencies
    detect_cycles(&config.processes)?;

    Ok(())
}

/// Detect cyclic dependencies using DFS
fn detect_cycles(processes: &HashMap<String, ProcessConfig>) -> Result<()> {
    let mut visited: HashMap<&str, u8> = HashMap::new(); // 0=unvisited, 1=in-progress, 2=done

    for name in processes.keys() {
        if visited.get(name.as_str()).copied().unwrap_or(0) == 0 {
            dfs_cycle_check(name, processes, &mut visited)?;
        }
    }

    Ok(())
}

fn dfs_cycle_check<'a>(
    node: &'a str,
    processes: &'a HashMap<String, ProcessConfig>,
    visited: &mut HashMap<&'a str, u8>,
) -> Result<()> {
    visited.insert(node, 1); // Mark as in-progress

    if let Some(process) = processes.get(node) {
        for dep in &process.depends_on {
            match visited.get(dep.as_str()).copied().unwrap_or(0) {
                1 => {
                    anyhow::bail!(
                        "Cyclic dependency detected: '{}' -> '{}' forms a cycle",
                        node,
                        dep
                    );
                }
                0 => {
                    dfs_cycle_check(dep, processes, visited)?;
                }
                _ => {} // Already fully visited
            }
        }
    }

    visited.insert(node, 2); // Mark as done
    Ok(())
}

/// Topological sort: returns process names in dependency order (dependencies first)
pub fn topological_sort(processes: &HashMap<String, ProcessConfig>) -> Result<Vec<String>> {
    let mut result = Vec::new();
    let mut visited: HashMap<&str, bool> = HashMap::new();

    for name in processes.keys() {
        if !visited.contains_key(name.as_str()) {
            topo_dfs(name, processes, &mut visited, &mut result)?;
        }
    }

    Ok(result)
}

fn topo_dfs<'a>(
    node: &'a str,
    processes: &'a HashMap<String, ProcessConfig>,
    visited: &mut HashMap<&'a str, bool>,
    result: &mut Vec<String>,
) -> Result<()> {
    visited.insert(node, true);

    if let Some(process) = processes.get(node) {
        for dep in &process.depends_on {
            if !visited.contains_key(dep.as_str()) {
                topo_dfs(dep, processes, visited, result)?;
            }
        }
    }

    result.push(node.to_string());
    Ok(())
}

// =============================================================================
// Default Value Functions
// =============================================================================

fn default_true() -> bool {
    true
}
fn default_working_dir() -> String {
    ".".to_string()
}
fn default_restart_delay() -> u64 {
    5
}
fn default_log_activity_timeout() -> u64 {
    300
}
fn default_max_memory_mb() -> u64 {
    2048
}
fn default_max_cpu_percent() -> f32 {
    90.0
}
fn default_check_interval() -> u64 {
    30
}
fn default_http_timeout() -> u64 {
    5
}
fn default_http_interval() -> u64 {
    60
}
fn default_ready_method() -> ReadyMethod {
    ReadyMethod::Time
}
fn default_ready_timeout() -> u64 {
    60
}
fn default_max_restarts() -> u32 {
    5
}
fn default_restart_window() -> u64 {
    300
}
fn default_backoff_strategy() -> String {
    "exponential".to_string()
}
fn default_initial_backoff() -> u64 {
    1
}
fn default_max_backoff() -> u64 {
    60
}
fn default_give_up_action() -> String {
    "notify".to_string()
}
fn default_grace_period() -> u64 {
    30
}
fn default_log_output() -> String {
    "guardian.log".to_string()
}
fn default_log_level() -> String {
    "info".to_string()
}
fn default_rotation() -> String {
    "daily".to_string()
}
fn default_log_max_size() -> String {
    "50MB".to_string()
}
fn default_log_max_files() -> u32 {
    7
}
fn default_log_format() -> String {
    "json".to_string()
}
fn default_timestamp_format() -> String {
    "rfc3339".to_string()
}
fn default_token_threshold() -> u64 {
    30000
}
fn default_schedule() -> String {
    "daily".to_string()
}
fn default_schedule_time() -> String {
    "03:00".to_string()
}
fn default_compression_target() -> f64 {
    0.5
}
fn default_min_compression_ratio() -> f64 {
    5.0
}
fn default_compression_method() -> String {
    "llm".to_string()
}
fn default_max_retries() -> u32 {
    3
}
fn default_retry_delay() -> u64 {
    5
}
fn default_smtp_port() -> u16 {
    587
}
fn default_supervisor_interval() -> u64 {
    5
}
fn default_spawn_timeout() -> u64 {
    30
}
fn default_shutdown_grace_period() -> u64 {
    60
}
fn default_pid_file() -> String {
    "guardian.pid".to_string()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn minimal_toml() -> &'static str {
        r#"
[processes.test]
name = "test"
command = "echo"
args = ["hello"]
"#
    }

    #[test]
    fn test_load_valid_config() {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", minimal_toml()).unwrap();

        let config = load_config(file.path()).unwrap();
        assert!(config.processes.contains_key("test"));
        assert_eq!(config.processes["test"].command, "echo");
    }

    #[test]
    fn test_invalid_toml_returns_error() {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "invalid toml {{{{").unwrap();

        let result = load_config(file.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_processes_returns_error() {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "[processes]").unwrap();

        let result = load_config(file.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_cyclic_dependency_detected() {
        let toml_str = r#"
[processes.a]
name = "a"
command = "echo"
depends_on = ["b"]

[processes.b]
name = "b"
command = "echo"
depends_on = ["a"]
"#;
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", toml_str).unwrap();

        let result = load_config(file.path());
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Cyclic dependency"));
    }

    #[test]
    fn test_missing_dependency_detected() {
        let toml_str = r#"
[processes.a]
name = "a"
command = "echo"
depends_on = ["nonexistent"]
"#;
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", toml_str).unwrap();

        let result = load_config(file.path());
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("nonexistent"));
    }

    #[test]
    fn test_default_values() {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", minimal_toml()).unwrap();

        let config = load_config(file.path()).unwrap();
        let process = &config.processes["test"];

        assert!(process.auto_restart);
        assert_eq!(process.restart_delay, 5);
        assert!(process.health.process_alive);
        assert_eq!(process.health.max_memory_mb, 2048);
    }

    #[test]
    fn test_topological_sort_simple() {
        let toml_str = r#"
[processes.openclaw]
name = "openclaw"
command = "openclaw"

[processes.oc-memory]
name = "oc-memory"
command = "python"
depends_on = ["openclaw"]
"#;
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", toml_str).unwrap();

        let config = load_config(file.path()).unwrap();
        let order = topological_sort(&config.processes).unwrap();

        let openclaw_idx = order.iter().position(|x| x == "openclaw").unwrap();
        let memory_idx = order.iter().position(|x| x == "oc-memory").unwrap();
        assert!(openclaw_idx < memory_idx);
    }

    #[test]
    fn test_expand_tilde() {
        let home = "/home/user";
        assert_eq!(expand_tilde("~/test", home), "/home/user/test");
        assert_eq!(expand_tilde("/absolute/path", home), "/absolute/path");
        assert_eq!(expand_tilde("relative/path", home), "relative/path");
    }
}
