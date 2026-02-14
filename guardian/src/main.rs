#![allow(dead_code)]

mod compression;
mod config;
mod health;
mod log_rotation;
mod logger;
mod notification;
mod process;
mod recovery;

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Cell, Color, Table};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tracing::{error, info, warn};

use crate::compression::CompressionManager;
use crate::config::{load_config, GuardianConfig};
use crate::health::{HealthChecker, HealthStatus};
use crate::log_rotation::LogRotator;
use crate::notification::{EventType, NotificationEvent, NotificationManager, Severity};
use crate::process::{process_matches_with_args, ProcessManager, ProcessState};
use crate::recovery::RecoveryEngine;

// =============================================================================
// CLI Definition
// =============================================================================

#[derive(Parser)]
#[command(
    name = "oc-guardian",
    about = "OpenClaw Process Guardian - Manages OpenClaw and OC-Memory processes",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to guardian.toml configuration file
    #[arg(short, long, default_value = "guardian.toml")]
    config: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Start everything: openclaw → oc-guardian → oc-memory (one command)
    Up,

    /// Stop everything: oc-memory → oc-guardian → openclaw (reverse order)
    Down,

    /// Start guardian supervisor only (managed processes)
    Start,

    /// Stop guardian supervisor only (managed processes)
    Stop,

    /// Restart a specific process or all processes
    Restart {
        /// Process name to restart (all if omitted)
        process: Option<String>,
    },

    /// Show status of all managed processes
    Status,

    /// View logs for a specific process
    Logs {
        /// Process name to view logs for
        process: Option<String>,

        /// Follow log output (like tail -f)
        #[arg(short, long)]
        follow: bool,

        /// Number of last lines to show
        #[arg(short = 'n', long, default_value = "50")]
        tail: usize,
    },
}

// =============================================================================
// Main
// =============================================================================

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load configuration
    let config_path = PathBuf::from(&cli.config);
    let config = load_config(&config_path)?;

    // Initialize logging
    logger::init_logging(&config.logging)?;

    info!("OC-Guardian starting with config: {}", cli.config);

    // Execute command
    match cli.command {
        Commands::Up => handle_up(config).await?,
        Commands::Down => handle_down(config).await?,
        Commands::Start => handle_start(config).await?,
        Commands::Stop => handle_stop(config).await?,
        Commands::Restart { process } => handle_restart(config, process).await?,
        Commands::Status => handle_status(config).await?,
        Commands::Logs {
            process,
            follow,
            tail,
        } => handle_logs(config, process, follow, tail).await?,
    }

    Ok(())
}

// =============================================================================
// Command Handlers
// =============================================================================

// =============================================================================
// Up / Down — Orchestrate everything with one command
// =============================================================================

async fn handle_up(config: GuardianConfig) -> Result<()> {
    println!("{}", "=== OC-Guardian Up ===".green().bold());

    // Step 1: Start externally managed processes (managed=false) directly
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();

    for (name, proc_config) in &config.processes {
        if proc_config.managed {
            continue; // guardian will handle these
        }

        let cmd_name = std::path::Path::new(&proc_config.command)
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or(&proc_config.command)
            .to_string();

        // Check if already running
        let already_running = sys.processes().values().any(|p| process_matches_with_args(p, &cmd_name, &proc_config.args));

        if already_running {
            println!(
                "  {} {} (already running)",
                "✓".green(),
                name
            );
        } else {
            println!(
                "  {} Starting {}...",
                "→".cyan(),
                name
            );

            let mut cmd = std::process::Command::new(&proc_config.command);
            cmd.args(&proc_config.args);

            let work_dir = &proc_config.working_dir;
            if work_dir != "." && Path::new(work_dir).exists() {
                cmd.current_dir(work_dir);
            }

            for (key, value) in &proc_config.env {
                cmd.env(key, value);
            }

            cmd.stdout(std::process::Stdio::null());
            cmd.stderr(std::process::Stdio::null());

            match cmd.spawn() {
                Ok(child) => {
                    println!(
                        "  {} {} started (PID: {})",
                        "✓".green(),
                        name,
                        child.id()
                    );
                }
                Err(e) => {
                    println!(
                        "  {} Failed to start {}: {}",
                        "✗".red(),
                        name,
                        e
                    );
                    return Err(e.into());
                }
            }

            // Wait for ready
            let wait_secs = proc_config.ready.timeout.min(10);
            println!(
                "  {} Waiting {}s for {} to initialize...",
                "…".dimmed(),
                wait_secs,
                name
            );
            tokio::time::sleep(Duration::from_secs(wait_secs)).await;
        }
    }

    // Step 2: Start guardian (which starts managed processes like oc-memory)
    println!(
        "  {} Starting guardian supervisor...",
        "→".cyan()
    );

    handle_start(config).await
}

async fn handle_down(config: GuardianConfig) -> Result<()> {
    println!("{}", "=== OC-Guardian Down ===".yellow().bold());

    // Step 1: Stop guardian (which stops managed processes like oc-memory)
    println!(
        "  {} Stopping guardian + managed processes...",
        "→".yellow()
    );
    handle_stop(config.clone()).await?;

    // Step 2: Stop externally managed processes (managed=false)
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();

    for (name, proc_config) in &config.processes {
        if proc_config.managed {
            continue; // already stopped by handle_stop
        }

        let cmd_name = std::path::Path::new(&proc_config.command)
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or(&proc_config.command)
            .to_string();

        let mut found = false;
        // Kill ALL matching processes (parent + child, e.g., openclaw + openclaw-gateway)
        for (_pid, process) in sys.processes() {
            if process_matches_with_args(process, &cmd_name, &proc_config.args) {
                info!("Stopping {} process (PID: {})", name, _pid);
                process.kill();
                found = true;
            }
        }

        if found {
            println!("  {} {} stopped", "✓".green(), name);
        } else {
            println!(
                "  {} {} (not running)",
                "—".dimmed(),
                name
            );
        }
    }

    println!("{}", "All processes stopped.".green().bold());
    Ok(())
}

// =============================================================================
// Start / Stop — Guardian supervisor only
// =============================================================================

async fn handle_start(config: GuardianConfig) -> Result<()> {
    let pid_path = resolve_pid_path(&config);

    // Check for existing guardian process (duplicate prevention)
    if let Some(existing_pid) = read_pid_file(&pid_path) {
        if is_process_alive(existing_pid) {
            println!(
                "{}",
                format!(
                    "OC-Guardian is already running (PID: {}). Use 'oc-guardian stop' first.",
                    existing_pid
                )
                .red()
                .bold()
            );
            return Ok(());
        }
        // Stale PID file — clean up
        info!(
            "Removing stale PID file (PID {} no longer running)",
            existing_pid
        );
        let _ = std::fs::remove_file(&pid_path);
    }

    // Write our PID
    write_pid_file(&pid_path)?;

    // Install signal handlers EARLY (before start_all, which may block)
    let running = Arc::new(Mutex::new(true));
    let running_clone = running.clone();
    let running_clone2 = running.clone();

    // SIGINT (Ctrl+C)
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.ok();
        info!("Received shutdown signal (SIGINT)");
        println!(
            "\n{}",
            "Shutting down gracefully...".yellow().bold()
        );
        *running_clone.lock().await = false;
    });

    // SIGTERM (from oc-guardian stop / launchctl)
    tokio::spawn(async move {
        let mut sigterm =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                .expect("Failed to register SIGTERM handler");
        sigterm.recv().await;
        info!("Received shutdown signal (SIGTERM)");
        println!(
            "\n{}",
            "Shutting down gracefully (SIGTERM)...".yellow().bold()
        );
        *running_clone2.lock().await = false;
    });

    println!("{}", "Starting OC-Guardian...".green().bold());

    let manager = ProcessManager::new(config.clone());

    // Start all processes in dependency order (passes running flag for early abort)
    manager.start_all_with_flag(running.clone()).await?;

    println!("{}", "All processes started successfully!".green().bold());

    // Enter supervisor loop (reuses same running flag)
    let result = supervisor_loop(config, manager, running).await;

    // Clean up PID file on exit
    let _ = std::fs::remove_file(&pid_path);

    result
}

async fn handle_stop(config: GuardianConfig) -> Result<()> {
    println!("{}", "Stopping OC-Guardian...".yellow().bold());

    let pid_path = resolve_pid_path(&config);

    // First, stop the guardian supervisor process itself via PID file
    if let Some(guardian_pid) = read_pid_file(&pid_path) {
        if is_process_alive(guardian_pid) {
            info!("Sending SIGTERM to guardian process (PID: {})", guardian_pid);
            unsafe {
                libc::kill(guardian_pid as i32, libc::SIGTERM);
            }
            // Wait briefly for guardian to shut down gracefully
            // (it will stop its managed child processes during shutdown)
            tokio::time::sleep(Duration::from_secs(3)).await;

            if is_process_alive(guardian_pid) {
                warn!("Guardian still running, sending SIGKILL");
                unsafe {
                    libc::kill(guardian_pid as i32, libc::SIGKILL);
                }
            }
        }
        let _ = std::fs::remove_file(&pid_path);
    }

    // Then stop any remaining managed processes (not externally managed ones)
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();
    let mut stopped = 0;

    for (name, proc_config) in &config.processes {
        // Skip externally managed processes — don't stop them
        if !proc_config.managed {
            info!(
                "Skipping externally managed process '{}' (managed=false)",
                name
            );
            continue;
        }

        let cmd_name = std::path::Path::new(&proc_config.command)
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or(&proc_config.command)
            .to_string();

        for (_pid, process) in sys.processes() {
            if process_matches_with_args(process, &cmd_name, &proc_config.args) {
                info!("Stopping process '{}' (PID: {})", name, _pid);
                process.kill();
                stopped += 1;
                break;
            }
        }
    }

    if stopped == 0 {
        println!("{}", "Guardian stopped. No additional managed processes found.".yellow());
    } else {
        println!(
            "{}",
            format!("Guardian stopped. {} managed process(es) stopped.", stopped).green()
        );
    }

    Ok(())
}

async fn handle_restart(config: GuardianConfig, process: Option<String>) -> Result<()> {
    let manager = ProcessManager::new(config);

    match process {
        Some(name) => {
            println!("Restarting process '{}'...", name);
            manager.restart_process(&name).await?;
            println!("{}", format!("Process '{}' restarted.", name).green());
        }
        None => {
            println!("{}", "Restarting all processes...".yellow());
            manager.stop_all().await?;
            manager.start_all().await?;
            println!("{}", "All processes restarted.".green());
        }
    }

    Ok(())
}

async fn handle_status(config: GuardianConfig) -> Result<()> {
    // Discover actual running processes via sysinfo
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new("Name").fg(Color::White),
            Cell::new("Status").fg(Color::White),
            Cell::new("PID").fg(Color::White),
            Cell::new("Command").fg(Color::White),
        ]);

    for (name, proc_config) in &config.processes {
        let cmd_name = std::path::Path::new(&proc_config.command)
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or(&proc_config.command)
            .to_string();

        let mut found = false;
        for (pid, process) in sys.processes() {
            if process_matches_with_args(process, &cmd_name, &proc_config.args) {
                table.add_row(vec![
                    Cell::new(name),
                    Cell::new("online").fg(Color::Green),
                    Cell::new(pid.to_string()),
                    Cell::new(&proc_config.command),
                ]);
                found = true;
                break;
            }
        }

        if !found {
            table.add_row(vec![
                Cell::new(name),
                Cell::new("stopped").fg(Color::DarkGrey),
                Cell::new("-"),
                Cell::new(&proc_config.command),
            ]);
        }
    }

    println!("{}", table);
    Ok(())
}

async fn handle_logs(
    config: GuardianConfig,
    process: Option<String>,
    follow: bool,
    tail: usize,
) -> Result<()> {
    // Determine which log file to read
    let log_file = match process {
        Some(ref name) => {
            if let Some(proc_config) = config.processes.get(name) {
                proc_config
                    .health
                    .log_file
                    .clone()
                    .unwrap_or_else(|| format!("{}.log", name))
            } else {
                anyhow::bail!("Process '{}' not found", name);
            }
        }
        None => config.logging.output.clone(),
    };

    let path = std::path::Path::new(&log_file);
    if !path.exists() {
        println!("Log file not found: {}", log_file);
        return Ok(());
    }

    // Read last N lines
    let content = std::fs::read_to_string(path)?;
    let lines: Vec<&str> = content.lines().collect();
    let start = if lines.len() > tail {
        lines.len() - tail
    } else {
        0
    };

    for line in &lines[start..] {
        println!("{}", line);
    }

    if follow {
        println!("{}", "--- Following log output (Ctrl+C to stop) ---".dimmed());

        // Simple follow: poll for new content
        let mut last_len = content.len();
        loop {
            tokio::time::sleep(Duration::from_millis(500)).await;

            if let Ok(new_content) = std::fs::read_to_string(path) {
                if new_content.len() > last_len {
                    let new_part = &new_content[last_len..];
                    print!("{}", new_part);
                    last_len = new_content.len();
                }
            }
        }
    }

    Ok(())
}

// =============================================================================
// PID File Management
// =============================================================================

/// Resolve the PID file path from config (relative to working directory)
fn resolve_pid_path(config: &GuardianConfig) -> PathBuf {
    let pid_file = &config.advanced.pid_file;
    let path = Path::new(pid_file);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        // Relative to /usr/local/etc/oc-guardian/ (working directory)
        PathBuf::from("/usr/local/etc/oc-guardian").join(pid_file)
    }
}

/// Read PID from file, returning None if file doesn't exist or is invalid
fn read_pid_file(path: &Path) -> Option<u32> {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|s| s.trim().parse::<u32>().ok())
}

/// Write current process PID to file
fn write_pid_file(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, std::process::id().to_string())?;
    info!("PID file written: {} (PID: {})", path.display(), std::process::id());
    Ok(())
}

/// Check if a process with the given PID is still alive
fn is_process_alive(pid: u32) -> bool {
    // kill(pid, 0) checks if process exists without sending a signal
    unsafe { libc::kill(pid as i32, 0) == 0 }
}

// =============================================================================
// Supervisor Main Loop (Phase 3: with log rotation, compression, sleep, notifications)
// =============================================================================

async fn supervisor_loop(
    config: GuardianConfig,
    manager: ProcessManager,
    running: Arc<Mutex<bool>>,
) -> Result<()> {
    let interval = Duration::from_secs(config.advanced.supervisor_interval);
    let mut health_checker = HealthChecker::new();
    let mut recovery_engine = RecoveryEngine::new(config.recovery.clone());

    // Phase 3 subsystems
    let mut log_rotator = LogRotator::new(config.logging.clone());
    let mut compression_manager =
        CompressionManager::new(config.memory.compression.clone());
    let mut notifier = NotificationManager::new(config.notifications.clone());

    info!("Supervisor loop started (interval: {:?})", interval);
    println!(
        "{}",
        "Supervisor active. Press Ctrl+C to stop.".dimmed()
    );

    // Send startup notification
    let _ = notifier
        .notify(&NotificationEvent {
            event_type: EventType::GuardianStartup,
            process_name: "guardian".to_string(),
            message: "OC-Guardian supervisor started".to_string(),
            severity: Severity::Info,
        })
        .await;

    let mut check_count: u64 = 0;
    let log_rotation_interval = 3600; // 1 hour in seconds

    loop {
        // Check if we should stop
        if !*running.lock().await {
            info!("Supervisor shutting down...");

            // Stop all processes in reverse dependency order
            println!("{}", "Stopping all processes in dependency order...".yellow());
            manager.stop_all().await?;

            // Log final recovery stats
            let stats = recovery_engine.stats();
            info!(
                "Final recovery stats: total={}, successful={}, failed={}",
                stats.total_recoveries, stats.successful, stats.failed
            );

            // Log compression stats
            let comp_history = compression_manager.history();
            if comp_history.total_compressions() > 0 {
                info!(
                    "Compression stats: total={}, successful={}, avg_ratio={:.1}x",
                    comp_history.total_compressions(),
                    comp_history.successful_compressions(),
                    comp_history.average_ratio()
                );
            }

            // Send shutdown notification
            let _ = notifier
                .notify(&NotificationEvent {
                    event_type: EventType::GuardianShutdown,
                    process_name: "guardian".to_string(),
                    message: "OC-Guardian supervisor stopped".to_string(),
                    severity: Severity::Info,
                })
                .await;

            println!("{}", "All processes stopped. Goodbye!".green());
            break;
        }

        check_count += 1;

        // Health check each process
        for (name, proc_arc) in &manager.processes {
            let proc = proc_arc.lock().await;

            if proc.state != ProcessState::Running {
                // Check if process was supposed to be running but died
                if proc.state == ProcessState::Failed && proc.config.auto_restart {
                    let restart_count = proc.restart_count;
                    drop(proc);

                    info!("Process '{}' is in Failed state, attempting restart", name);

                    // Send crash notification
                    let _ = notifier
                        .notify(&NotificationEvent {
                            event_type: EventType::ProcessCrash,
                            process_name: name.clone(),
                            message: format!("Process '{}' crashed, attempting recovery", name),
                            severity: Severity::Critical,
                        })
                        .await;

                    if let Some(action) = recovery_engine.evaluate(
                        name,
                        &health::HealthCheckResult {
                            process_name: name.clone(),
                            status: HealthStatus::Unhealthy("Process failed".to_string()),
                            level_results: vec![health::LevelResult {
                                level: 1,
                                name: "Process Alive".to_string(),
                                passed: false,
                                message: "Process in Failed state".to_string(),
                            }],
                            checked_at: std::time::Instant::now(),
                        },
                        restart_count,
                    ) {
                        if let Err(e) = recovery_engine.execute(&action, name, &manager).await {
                            error!("Recovery failed for '{}': {}", name, e);
                        }
                    }
                    continue;
                }

                continue;
            }

            let health_config = proc.config.health.clone();
            let pid = proc.pid;
            let restart_count = proc.restart_count;
            drop(proc);

            // Run health checks
            let health_result = health_checker
                .check_process(name, pid, &health_config)
                .await;

            // Evaluate recovery if needed
            if health_result.status != HealthStatus::Healthy {
                warn!(
                    "Process '{}' health: {}",
                    name, health_result.status
                );

                // Notify about health check failure
                let _ = notifier
                    .notify(&NotificationEvent {
                        event_type: EventType::HealthCheckFailed,
                        process_name: name.clone(),
                        message: format!(
                            "Health check failed: {}",
                            health_result.status
                        ),
                        severity: Severity::Warning,
                    })
                    .await;

                if let Some(action) = recovery_engine.evaluate(name, &health_result, restart_count)
                {
                    info!("Recovery action for '{}': {}", name, action);

                    if let Err(e) = recovery_engine.execute(&action, name, &manager).await {
                        error!("Recovery failed for '{}': {}", name, e);
                    }
                }
            }
        }

        // Log rotation check (every hour)
        if log_rotator.should_check(log_rotation_interval) {
            match log_rotator.rotate_if_needed() {
                Ok(stats) => {
                    if stats.files_rotated > 0 {
                        info!(
                            "Log rotation: {} files rotated ({} checked, {} errors)",
                            stats.files_rotated, stats.files_checked, stats.errors
                        );
                    }
                }
                Err(e) => {
                    error!("Log rotation failed: {}", e);
                }
            }
        }

        // Memory compression check (every check cycle)
        match compression_manager.check_and_compress().await {
            Ok(Some(result)) => {
                info!(
                    "Compression completed: {:.1}x ratio, {} tokens -> {} tokens",
                    result.compression_ratio, result.tokens_before, result.tokens_after
                );

                let _ = notifier
                    .notify(&NotificationEvent {
                        event_type: EventType::CompressionComplete,
                        process_name: "guardian".to_string(),
                        message: format!(
                            "Memory compression: {:.1}x ratio ({} -> {} tokens)",
                            result.compression_ratio, result.tokens_before, result.tokens_after
                        ),
                        severity: Severity::Info,
                    })
                    .await;
            }
            Ok(None) => {}
            Err(e) => {
                error!("Compression check failed: {}", e);
            }
        }

        // Periodic stats logging (every 60 checks)
        if check_count % 60 == 0 {
            let stats = recovery_engine.stats();
            if stats.total_recoveries > 0 {
                info!(
                    "Recovery stats: total={}, ok={}, failed={}, scenarios={:?}",
                    stats.total_recoveries, stats.successful, stats.failed, stats.by_scenario
                );
            }

            let notif_stats = notifier.stats();
            if notif_stats.total_sent > 0 {
                info!("Notification stats: total_sent={}", notif_stats.total_sent);
            }
        }

        tokio::time::sleep(interval).await;
    }

    Ok(())
}
