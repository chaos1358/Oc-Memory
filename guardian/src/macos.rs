use anyhow::Result;
use tracing::{info, warn};


use crate::config::MacOsConfig;

// =============================================================================
// macOS Sleep Prevention (Sprint 3.5)
// =============================================================================

/// Manages macOS sleep prevention using caffeinate
pub struct SleepPreventer {
    config: MacOsConfig,
    #[cfg(target_os = "macos")]
    caffeinate_child: Option<tokio::process::Child>,
    #[cfg(not(target_os = "macos"))]
    _phantom: std::marker::PhantomData<()>,
}

impl SleepPreventer {
    pub fn new(config: MacOsConfig) -> Self {
        Self {
            config,
            #[cfg(target_os = "macos")]
            caffeinate_child: None,
            #[cfg(not(target_os = "macos"))]
            _phantom: std::marker::PhantomData,
        }
    }

    /// Start sleep prevention if enabled and on macOS
    pub async fn start(&mut self) -> Result<()> {
        if !self.config.prevent_sleep {
            info!("Sleep prevention is disabled");
            return Ok(());
        }

        #[cfg(target_os = "macos")]
        {
            if self.config.use_caffeinate {
                self.start_caffeinate().await?;
            } else {
                self.start_pmset().await?;
            }
        }

        #[cfg(not(target_os = "macos"))]
        {
            if self.config.prevent_sleep {
                info!("Sleep prevention is only supported on macOS (current platform ignored)");
            }
        }

        Ok(())
    }

    /// Stop sleep prevention and restore settings
    pub async fn stop(&mut self) -> Result<()> {
        if !self.config.prevent_sleep {
            return Ok(());
        }

        #[cfg(target_os = "macos")]
        {
            self.stop_caffeinate().await?;

            if self.config.restore_sleep_on_exit && !self.config.use_caffeinate {
                self.restore_pmset().await?;
            }
        }

        Ok(())
    }

    /// Check if sleep prevention is active
    pub fn is_active(&self) -> bool {
        #[cfg(target_os = "macos")]
        {
            self.caffeinate_child.is_some()
        }
        #[cfg(not(target_os = "macos"))]
        {
            false
        }
    }

    // =========================================================================
    // macOS-specific implementations
    // =========================================================================

    /// Start caffeinate process (-d: prevent display sleep, -i: prevent idle sleep)
    #[cfg(target_os = "macos")]
    async fn start_caffeinate(&mut self) -> Result<()> {
        use tokio::process::Command;

        info!("Starting caffeinate to prevent sleep...");

        let child = Command::new("caffeinate")
            .arg("-di") // prevent display and idle sleep
            .spawn()?;

        let pid = child.id();
        info!(
            "caffeinate started (PID: {})",
            pid.map(|p| p.to_string())
                .unwrap_or_else(|| "unknown".to_string())
        );

        self.caffeinate_child = Some(child);
        Ok(())
    }

    /// Stop caffeinate process
    #[cfg(target_os = "macos")]
    async fn stop_caffeinate(&mut self) -> Result<()> {
        if let Some(ref mut child) = self.caffeinate_child {
            info!("Stopping caffeinate...");
            let _ = child.kill().await;
            let _ = child.wait().await;
            info!("caffeinate stopped");
        }
        self.caffeinate_child = None;
        Ok(())
    }

    /// Use pmset to prevent sleep (requires sudo)
    #[cfg(target_os = "macos")]
    async fn start_pmset(&self) -> Result<()> {
        use tokio::process::Command;

        info!("Using pmset to disable sleep (may require sudo)...");

        let output = Command::new("sudo")
            .args(["pmset", "-c", "disablesleep", "1"])
            .output()
            .await?;

        if output.status.success() {
            info!("pmset: sleep disabled");
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            warn!("pmset failed (may need sudo): {}", stderr);
        }

        Ok(())
    }

    /// Restore pmset sleep settings
    #[cfg(target_os = "macos")]
    async fn restore_pmset(&self) -> Result<()> {
        use tokio::process::Command;

        info!("Restoring sleep settings with pmset...");

        let output = Command::new("sudo")
            .args(["pmset", "-c", "disablesleep", "0"])
            .output()
            .await?;

        if output.status.success() {
            info!("pmset: sleep restored");
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            warn!("pmset restore failed: {}", stderr);
        }

        Ok(())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sleep_preventer_disabled() {
        let config = MacOsConfig {
            prevent_sleep: false,
            use_caffeinate: true,
            restore_sleep_on_exit: true,
        };
        let preventer = SleepPreventer::new(config);
        assert!(!preventer.is_active());
    }

    #[test]
    fn test_sleep_preventer_non_macos() {
        let config = MacOsConfig {
            prevent_sleep: true,
            use_caffeinate: true,
            restore_sleep_on_exit: true,
        };
        let preventer = SleepPreventer::new(config);

        // On non-macOS, should never be active
        #[cfg(not(target_os = "macos"))]
        assert!(!preventer.is_active());
    }

    #[tokio::test]
    async fn test_start_when_disabled() {
        let config = MacOsConfig {
            prevent_sleep: false,
            use_caffeinate: true,
            restore_sleep_on_exit: true,
        };
        let mut preventer = SleepPreventer::new(config);
        preventer.start().await.unwrap();
        assert!(!preventer.is_active());
    }

    #[tokio::test]
    async fn test_stop_when_disabled() {
        let config = MacOsConfig {
            prevent_sleep: false,
            use_caffeinate: true,
            restore_sleep_on_exit: true,
        };
        let mut preventer = SleepPreventer::new(config);
        // Should not error
        preventer.stop().await.unwrap();
    }
}
