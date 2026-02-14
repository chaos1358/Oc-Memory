use anyhow::{Context, Result};
use chrono::Utc;
use serde::Serialize;
use std::collections::HashMap;
use tracing::{error, info};

use crate::config::{EmailConfig, NotificationConfig};

// =============================================================================
// Notification Manager (Sprint 3.6)
// =============================================================================

pub struct NotificationManager {
    config: NotificationConfig,
    send_count: u64,
    last_error: Option<String>,
}

impl NotificationManager {
    pub fn new(config: NotificationConfig) -> Self {
        Self {
            config,
            send_count: 0,
            last_error: None,
        }
    }

    /// Send notification for an event
    pub async fn notify(&mut self, event: &NotificationEvent) -> Result<()> {
        if !self.config.enabled {
            return Ok(());
        }

        // Send email notification
        if let Some(ref email_config) = self.config.email {
            // Check if this event type should trigger email
            if email_config.events.is_empty()
                || email_config.events.contains(&event.event_type.to_string())
            {
                match self.send_email(email_config, event).await {
                    Ok(_) => {
                        self.send_count += 1;
                        info!(
                            "Notification sent for event '{}' on process '{}'",
                            event.event_type, event.process_name
                        );
                    }
                    Err(e) => {
                        let err_msg = format!("Email send failed: {}", e);
                        self.last_error = Some(err_msg.clone());
                        // Don't propagate email errors - just log them
                        error!("{}", err_msg);
                    }
                }
            }
        }

        Ok(())
    }

    /// Send email notification via SMTP
    async fn send_email(
        &self,
        email_config: &EmailConfig,
        event: &NotificationEvent,
    ) -> Result<()> {
        // Build email content from template
        let (subject, body) = self.build_email_content(email_config, event);

        // Use tokio::process to call a system mail command or Python script
        // This avoids adding lettre as a heavy dependency
        // Instead, we use a lightweight SMTP approach via command-line tools
        let to_addresses = email_config.to.join(",");

        // Try Python smtplib first (most portable)
        let smtp_server = &email_config.smtp_server;
        let smtp_port = email_config.smtp_port;
        let from_addr = &email_config.from;
        let use_tls = email_config.smtp_tls;

        // Get SMTP credentials
        let smtp_user = email_config.smtp_user.clone().unwrap_or_default();
        let smtp_password = email_config
            .smtp_password_env
            .as_ref()
            .and_then(|env_var| std::env::var(env_var).ok())
            .unwrap_or_default();

        // Build Python one-liner for sending email
        let python_script = build_smtp_script(
            smtp_server,
            smtp_port,
            use_tls,
            &smtp_user,
            &smtp_password,
            from_addr,
            &to_addresses,
            &subject,
            &body,
        );

        let output = tokio::process::Command::new("python3")
            .arg("-c")
            .arg(&python_script)
            .output()
            .await;

        let output = match output {
            Ok(o) => o,
            Err(_) => {
                // Fallback to "python" if "python3" not found
                tokio::process::Command::new("python")
                    .arg("-c")
                    .arg(&python_script)
                    .output()
                    .await
                    .with_context(|| {
                        "Failed to execute email send command (tried python3 and python)"
                    })?
            }
        };

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Email send failed: {}", stderr);
        }

        Ok(())
    }

    /// Build email subject and body from template with variable substitution
    fn build_email_content(
        &self,
        email_config: &EmailConfig,
        event: &NotificationEvent,
    ) -> (String, String) {
        let mut vars: HashMap<&str, String> = HashMap::new();
        vars.insert("event_type", event.event_type.to_string());
        vars.insert("process_name", event.process_name.clone());
        vars.insert("message", event.message.clone());
        vars.insert("severity", event.severity.to_string());
        vars.insert("timestamp", Utc::now().to_rfc3339());
        vars.insert("hostname", hostname());

        // Use template if provided, otherwise defaults
        let subject = match email_config
            .template
            .as_ref()
            .and_then(|t| t.subject.as_ref())
        {
            Some(template) => substitute_vars(template, &vars),
            None => format!(
                "[OC-Guardian] {} - {} on {}",
                event.severity,
                event.event_type,
                event.process_name
            ),
        };

        let body = match email_config
            .template
            .as_ref()
            .and_then(|t| t.body.as_ref())
        {
            Some(template) => substitute_vars(template, &vars),
            None => format!(
                "OC-Guardian Alert\n\
                 ==================\n\
                 Event: {}\n\
                 Process: {}\n\
                 Severity: {}\n\
                 Time: {}\n\
                 Host: {}\n\
                 \n\
                 Message:\n\
                 {}\n\
                 \n\
                 ---\n\
                 Sent by OC-Guardian Process Manager",
                event.event_type,
                event.process_name,
                event.severity,
                Utc::now().to_rfc3339(),
                hostname(),
                event.message
            ),
        };

        (subject, body)
    }

    /// Get send statistics
    pub fn stats(&self) -> NotificationStats {
        NotificationStats {
            total_sent: self.send_count,
            last_error: self.last_error.clone(),
        }
    }
}

// =============================================================================
// Notification Event
// =============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct NotificationEvent {
    pub event_type: EventType,
    pub process_name: String,
    pub message: String,
    pub severity: Severity,
}

#[derive(Debug, Clone, Serialize)]
pub enum EventType {
    ProcessCrash,
    ProcessRestart,
    RecoveryAction,
    HealthCheckFailed,
    CompressionComplete,
    GuardianStartup,
    GuardianShutdown,
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::ProcessCrash => write!(f, "process_crash"),
            EventType::ProcessRestart => write!(f, "process_restart"),
            EventType::RecoveryAction => write!(f, "recovery_action"),
            EventType::HealthCheckFailed => write!(f, "health_check_failed"),
            EventType::CompressionComplete => write!(f, "compression_complete"),
            EventType::GuardianStartup => write!(f, "guardian_startup"),
            EventType::GuardianShutdown => write!(f, "guardian_shutdown"),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum Severity {
    Info,
    Warning,
    Critical,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Info => write!(f, "INFO"),
            Severity::Warning => write!(f, "WARNING"),
            Severity::Critical => write!(f, "CRITICAL"),
        }
    }
}

#[derive(Debug)]
pub struct NotificationStats {
    pub total_sent: u64,
    pub last_error: Option<String>,
}

// =============================================================================
// Helper Functions
// =============================================================================

/// Variable substitution in template strings
fn substitute_vars(template: &str, vars: &HashMap<&str, String>) -> String {
    let mut result = template.to_string();
    for (key, value) in vars {
        result = result.replace(&format!("{{{}}}", key), value);
    }
    result
}

/// Get system hostname
fn hostname() -> String {
    #[cfg(target_os = "windows")]
    {
        std::env::var("COMPUTERNAME").unwrap_or_else(|_| "unknown".to_string())
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::env::var("HOSTNAME")
            .or_else(|_| std::env::var("HOST"))
            .unwrap_or_else(|_| "unknown".to_string())
    }
}

/// Build Python SMTP script for email sending
fn build_smtp_script(
    server: &str,
    port: u16,
    use_tls: bool,
    user: &str,
    password: &str,
    from: &str,
    to: &str,
    subject: &str,
    body: &str,
) -> String {
    // Escape single quotes in strings for Python
    let subject_escaped = subject.replace('\'', "\\'");
    let body_escaped = body.replace('\'', "\\'").replace('\n', "\\n");
    let user_escaped = user.replace('\'', "\\'");
    let password_escaped = password.replace('\'', "\\'");

    format!(
        r#"
import smtplib
from email.mime.text import MIMEText
from email.mime.multipart import MIMEMultipart

msg = MIMEMultipart()
msg['From'] = '{from}'
msg['To'] = '{to}'
msg['Subject'] = '{subject}'
msg.attach(MIMEText('{body}', 'plain'))

try:
    server = smtplib.SMTP('{server}', {port})
    {tls}
    {auth}
    server.sendmail('{from}', '{to}'.split(','), msg.as_string())
    server.quit()
except Exception as e:
    import sys
    print(str(e), file=sys.stderr)
    sys.exit(1)
"#,
        from = from,
        to = to,
        subject = subject_escaped,
        body = body_escaped,
        server = server,
        port = port,
        tls = if use_tls { "server.starttls()" } else { "pass" },
        auth = if !user.is_empty() {
            format!("server.login('{}', '{}')", user_escaped, password_escaped)
        } else {
            "pass".to_string()
        },
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substitute_vars() {
        let mut vars = HashMap::new();
        vars.insert("name", "OpenClaw".to_string());
        vars.insert("event", "crash".to_string());

        let result = substitute_vars("Process {name} had {event}", &vars);
        assert_eq!(result, "Process OpenClaw had crash");
    }

    #[test]
    fn test_substitute_vars_no_match() {
        let vars = HashMap::new();
        let result = substitute_vars("No variables here", &vars);
        assert_eq!(result, "No variables here");
    }

    #[test]
    fn test_event_type_display() {
        assert_eq!(EventType::ProcessCrash.to_string(), "process_crash");
        assert_eq!(EventType::ProcessRestart.to_string(), "process_restart");
        assert_eq!(
            EventType::HealthCheckFailed.to_string(),
            "health_check_failed"
        );
    }

    #[test]
    fn test_severity_display() {
        assert_eq!(Severity::Info.to_string(), "INFO");
        assert_eq!(Severity::Warning.to_string(), "WARNING");
        assert_eq!(Severity::Critical.to_string(), "CRITICAL");
    }

    #[test]
    fn test_notification_disabled() {
        let config = NotificationConfig {
            enabled: false,
            email: None,
        };
        let manager = NotificationManager::new(config);
        assert_eq!(manager.stats().total_sent, 0);
    }

    #[test]
    fn test_email_content_default_template() {
        let config = NotificationConfig {
            enabled: true,
            email: Some(EmailConfig {
                smtp_server: "smtp.test.com".to_string(),
                smtp_port: 587,
                smtp_tls: true,
                smtp_user: None,
                smtp_password_env: None,
                from: "guardian@test.com".to_string(),
                to: vec!["admin@test.com".to_string()],
                events: vec![],
                template: None,
            }),
        };
        let manager = NotificationManager::new(config);

        let event = NotificationEvent {
            event_type: EventType::ProcessCrash,
            process_name: "openclaw".to_string(),
            message: "Process exited with code 1".to_string(),
            severity: Severity::Critical,
        };

        let email_config = manager.config.email.as_ref().unwrap();
        let (subject, body) = manager.build_email_content(email_config, &event);

        assert!(subject.contains("OC-Guardian"));
        assert!(subject.contains("CRITICAL"));
        assert!(body.contains("openclaw"));
        assert!(body.contains("Process exited"));
    }

    #[test]
    fn test_email_content_custom_template() {
        use crate::config::EmailTemplate;

        let config = NotificationConfig {
            enabled: true,
            email: Some(EmailConfig {
                smtp_server: "smtp.test.com".to_string(),
                smtp_port: 587,
                smtp_tls: true,
                smtp_user: None,
                smtp_password_env: None,
                from: "guardian@test.com".to_string(),
                to: vec!["admin@test.com".to_string()],
                events: vec![],
                template: Some(EmailTemplate {
                    subject: Some("Alert: {event_type} on {process_name}".to_string()),
                    body: Some("Process {process_name} triggered {event_type}: {message}".to_string()),
                }),
            }),
        };
        let manager = NotificationManager::new(config);

        let event = NotificationEvent {
            event_type: EventType::ProcessRestart,
            process_name: "oc-memory".to_string(),
            message: "Auto-restart triggered".to_string(),
            severity: Severity::Warning,
        };

        let email_config = manager.config.email.as_ref().unwrap();
        let (subject, body) = manager.build_email_content(email_config, &event);

        assert_eq!(subject, "Alert: process_restart on oc-memory");
        assert_eq!(
            body,
            "Process oc-memory triggered process_restart: Auto-restart triggered"
        );
    }

    #[test]
    fn test_hostname() {
        let host = hostname();
        assert!(!host.is_empty());
    }

    #[test]
    fn test_build_smtp_script() {
        let script = build_smtp_script(
            "smtp.gmail.com",
            587,
            true,
            "user@gmail.com",
            "password",
            "from@gmail.com",
            "to@gmail.com",
            "Test Subject",
            "Test Body",
        );
        assert!(script.contains("smtplib.SMTP"));
        assert!(script.contains("smtp.gmail.com"));
        assert!(script.contains("starttls"));
    }
}
