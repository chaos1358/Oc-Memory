use anyhow::Result;
use chrono::{Local, NaiveTime};
use serde::Serialize;
use std::path::Path;
use std::time::Instant;
use tokio::process::Command;
use tracing::{error, info, warn};

use crate::config::CompressionConfig;

// =============================================================================
// Compression Manager (Sprint 3.4)
// =============================================================================

pub struct CompressionManager {
    config: CompressionConfig,
    last_compression: Option<Instant>,
    last_schedule_date: Option<chrono::NaiveDate>,
    stats: CompressionHistory,
}

impl CompressionManager {
    pub fn new(config: CompressionConfig) -> Self {
        Self {
            config,
            last_compression: None,
            last_schedule_date: None,
            stats: CompressionHistory::default(),
        }
    }

    /// Check if compression should be triggered (dual trigger system)
    pub async fn check_and_compress(&mut self) -> Result<Option<CompressionResult>> {
        if !self.config.enabled {
            return Ok(None);
        }

        // Trigger 1: Token threshold exceeded
        if self.should_trigger_by_tokens().await? {
            info!("Compression triggered by token threshold");
            return self.run_compression("token_threshold").await.map(Some);
        }

        // Trigger 2: Scheduled time
        if self.should_trigger_by_schedule() {
            info!("Compression triggered by schedule");
            return self.run_compression("schedule").await.map(Some);
        }

        Ok(None)
    }

    /// Check if total token count across target files exceeds threshold
    async fn should_trigger_by_tokens(&self) -> Result<bool> {
        if self.config.target_files.is_empty() {
            return Ok(false);
        }

        let mut total_tokens: u64 = 0;

        for file_path in &self.config.target_files {
            let path = Path::new(file_path);
            if path.exists() {
                match tokio::fs::read_to_string(path).await {
                    Ok(content) => {
                        total_tokens += estimate_tokens(&content);
                    }
                    Err(e) => {
                        warn!("Failed to read target file {}: {}", file_path, e);
                    }
                }
            }
        }

        Ok(total_tokens >= self.config.token_threshold)
    }

    /// Check if scheduled compression time has arrived
    fn should_trigger_by_schedule(&self) -> bool {
        if !self.config.schedule_enabled {
            return false;
        }

        let now = Local::now();
        let today = now.date_naive();

        // Only trigger once per day
        if let Some(last_date) = self.last_schedule_date {
            if last_date == today {
                return false;
            }
        }

        // Parse schedule time (e.g., "03:00")
        if let Ok(schedule_time) = NaiveTime::parse_from_str(&self.config.schedule_time, "%H:%M") {
            let current_time = now.time();
            // Check if current time is within 5 minutes of schedule time
            let diff = current_time
                .signed_duration_since(schedule_time)
                .num_minutes();
            return diff >= 0 && diff < 5;
        }

        false
    }

    /// Run the compression process
    async fn run_compression(&mut self, trigger: &str) -> Result<CompressionResult> {
        let start = Instant::now();
        let mut result = CompressionResult {
            trigger: trigger.to_string(),
            files_processed: 0,
            tokens_before: 0,
            tokens_after: 0,
            compression_ratio: 0.0,
            duration_ms: 0,
            success: false,
        };

        // Calculate tokens before compression
        for file_path in &self.config.target_files {
            let path = Path::new(file_path);
            if path.exists() {
                if let Ok(content) = tokio::fs::read_to_string(path).await {
                    result.tokens_before += estimate_tokens(&content);
                    result.files_processed += 1;
                }
            }
        }

        // Smart skip: if below minimum threshold, skip compression
        if result.tokens_before < self.config.token_threshold / 2 {
            info!(
                "Skipping compression: tokens ({}) below smart skip threshold ({})",
                result.tokens_before,
                self.config.token_threshold / 2
            );
            result.duration_ms = start.elapsed().as_millis() as u64;
            return Ok(result);
        }

        // Execute compression based on method
        match self.config.method.as_str() {
            "llm" => {
                self.compress_with_llm(&mut result).await?;
            }
            "summarize" => {
                self.compress_with_summarize(&mut result).await?;
            }
            _ => {
                warn!("Unknown compression method: {}", self.config.method);
            }
        }

        // Calculate tokens after compression
        result.tokens_after = 0;
        for file_path in &self.config.target_files {
            let path = Path::new(file_path);
            if path.exists() {
                if let Ok(content) = tokio::fs::read_to_string(path).await {
                    result.tokens_after += estimate_tokens(&content);
                }
            }
        }

        // Calculate ratio
        if result.tokens_before > 0 {
            result.compression_ratio =
                result.tokens_before as f64 / result.tokens_after.max(1) as f64;
        }

        result.duration_ms = start.elapsed().as_millis() as u64;
        result.success = result.compression_ratio >= self.config.min_compression_ratio
            || result.tokens_after < result.tokens_before;

        // Update tracking
        self.last_compression = Some(Instant::now());
        if trigger == "schedule" {
            self.last_schedule_date = Some(Local::now().date_naive());
        }

        // Log stats
        if self.config.log_compression_stats {
            self.log_stats(&result);
        }

        self.stats.results.push(result.clone());

        Ok(result)
    }

    /// Compress using LLM (calls Python observer)
    async fn compress_with_llm(&self, result: &mut CompressionResult) -> Result<()> {
        // Build command to call Python observer
        let mut retries = 0;

        while retries < self.config.max_retries {
            let mut cmd = Command::new(&self.config.python_path);
            cmd.arg("-m").arg("lib.observer").arg("compress");

            // Pass target files
            for file in &self.config.target_files {
                cmd.arg("--target").arg(file);
            }

            // Pass LLM config
            if let Some(ref model) = self.config.llm_model {
                cmd.arg("--model").arg(model);
            }

            if let Some(ref api_key_env) = self.config.llm_api_key_env {
                if let Ok(key) = std::env::var(api_key_env) {
                    cmd.env("LLM_API_KEY", key);
                }
            }

            cmd.arg("--compression-target")
                .arg(self.config.compression_target.to_string());

            match cmd.output().await {
                Ok(output) => {
                    if output.status.success() {
                        result.success = true;
                        info!("LLM compression completed successfully");
                        return Ok(());
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        warn!(
                            "LLM compression attempt {} failed: {}",
                            retries + 1,
                            stderr
                        );
                    }
                }
                Err(e) => {
                    warn!(
                        "Failed to execute compression command (attempt {}): {}",
                        retries + 1,
                        e
                    );
                }
            }

            retries += 1;
            if retries < self.config.max_retries {
                tokio::time::sleep(std::time::Duration::from_secs(self.config.retry_delay)).await;
            }
        }

        error!(
            "LLM compression failed after {} attempts",
            self.config.max_retries
        );
        Ok(())
    }

    /// Compress using simple summarization (no LLM needed)
    async fn compress_with_summarize(&self, _result: &mut CompressionResult) -> Result<()> {
        // Simple text-based compression: remove duplicate lines, trim whitespace
        for file_path in &self.config.target_files {
            let path = Path::new(file_path);
            if !path.exists() {
                continue;
            }

            let content = tokio::fs::read_to_string(path).await?;
            let lines: Vec<&str> = content.lines().collect();

            // Remove consecutive duplicate lines
            let mut deduped = Vec::new();
            let mut last_line = "";
            for line in &lines {
                if *line != last_line || line.trim().is_empty() {
                    deduped.push(*line);
                    last_line = line;
                }
            }

            let new_content = deduped.join("\n");
            tokio::fs::write(path, new_content).await?;
        }

        Ok(())
    }

    /// Log compression statistics
    fn log_stats(&self, result: &CompressionResult) {
        info!(
            "Compression stats: trigger={}, files={}, tokens_before={}, tokens_after={}, ratio={:.1}x, duration={}ms, success={}",
            result.trigger,
            result.files_processed,
            result.tokens_before,
            result.tokens_after,
            result.compression_ratio,
            result.duration_ms,
            result.success
        );
    }

    /// Get compression history
    pub fn history(&self) -> &CompressionHistory {
        &self.stats
    }
}

// =============================================================================
// Token Estimation
// =============================================================================

/// Estimate token count from text (approximation: ~4 chars per token for English, ~2 for CJK)
fn estimate_tokens(text: &str) -> u64 {
    // Simple heuristic: split by whitespace and punctuation
    let word_count = text.split_whitespace().count() as u64;
    let char_count = text.chars().count() as u64;

    // Approximate: words * 1.3 (subword tokenization tends to produce ~1.3 tokens per word)
    // with a minimum of char_count / 4
    let by_words = (word_count as f64 * 1.3) as u64;
    let by_chars = char_count / 4;

    by_words.max(by_chars).max(1)
}

// =============================================================================
// Result Types
// =============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct CompressionResult {
    pub trigger: String,
    pub files_processed: u32,
    pub tokens_before: u64,
    pub tokens_after: u64,
    pub compression_ratio: f64,
    pub duration_ms: u64,
    pub success: bool,
}

#[derive(Debug, Default)]
pub struct CompressionHistory {
    pub results: Vec<CompressionResult>,
}

impl CompressionHistory {
    pub fn total_compressions(&self) -> usize {
        self.results.len()
    }

    pub fn successful_compressions(&self) -> usize {
        self.results.iter().filter(|r| r.success).count()
    }

    pub fn average_ratio(&self) -> f64 {
        let successful: Vec<&CompressionResult> =
            self.results.iter().filter(|r| r.success).collect();
        if successful.is_empty() {
            return 0.0;
        }
        let sum: f64 = successful.iter().map(|r| r.compression_ratio).sum();
        sum / successful.len() as f64
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_tokens_english() {
        let text = "Hello world, this is a test sentence with some words.";
        let tokens = estimate_tokens(text);
        assert!(tokens > 0);
        assert!(tokens < 100); // Reasonable range
    }

    #[test]
    fn test_estimate_tokens_empty() {
        let tokens = estimate_tokens("");
        assert_eq!(tokens, 1); // Minimum 1
    }

    #[test]
    fn test_estimate_tokens_long_text() {
        let text = "word ".repeat(1000);
        let tokens = estimate_tokens(&text);
        assert!(tokens >= 1000); // At least word count
    }

    #[test]
    fn test_compression_history_stats() {
        let mut history = CompressionHistory::default();
        assert_eq!(history.total_compressions(), 0);
        assert_eq!(history.average_ratio(), 0.0);

        history.results.push(CompressionResult {
            trigger: "test".to_string(),
            files_processed: 1,
            tokens_before: 10000,
            tokens_after: 2000,
            compression_ratio: 5.0,
            duration_ms: 100,
            success: true,
        });

        history.results.push(CompressionResult {
            trigger: "test".to_string(),
            files_processed: 1,
            tokens_before: 8000,
            tokens_after: 1000,
            compression_ratio: 8.0,
            duration_ms: 200,
            success: true,
        });

        assert_eq!(history.total_compressions(), 2);
        assert_eq!(history.successful_compressions(), 2);
        assert!((history.average_ratio() - 6.5).abs() < 0.01);
    }

    #[test]
    fn test_schedule_check_disabled() {
        let config = CompressionConfig {
            enabled: true,
            schedule_enabled: false,
            ..Default::default()
        };
        let manager = CompressionManager::new(config);
        assert!(!manager.should_trigger_by_schedule());
    }

    #[test]
    fn test_schedule_check_wrong_time() {
        let config = CompressionConfig {
            enabled: true,
            schedule_enabled: true,
            schedule_time: "99:99".to_string(), // Invalid time
            ..Default::default()
        };
        let manager = CompressionManager::new(config);
        assert!(!manager.should_trigger_by_schedule());
    }

    #[tokio::test]
    async fn test_compression_disabled() {
        let config = CompressionConfig {
            enabled: false,
            ..Default::default()
        };
        let mut manager = CompressionManager::new(config);
        let result = manager.check_and_compress().await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_no_target_files_no_trigger() {
        let config = CompressionConfig {
            enabled: true,
            target_files: vec![],
            ..Default::default()
        };
        let mut manager = CompressionManager::new(config);
        let result = manager.check_and_compress().await.unwrap();
        assert!(result.is_none());
    }
}
