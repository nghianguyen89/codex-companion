use std::path::PathBuf;

use crate::config::AppConfiguration;

pub fn operating_system() -> &'static str {
    match std::env::consts::OS {
        "windows" => "windows",
        "macos" => "macos",
        "linux" => "linux",
        _ => "unknown",
    }
}

pub fn codex_home() -> PathBuf {
    std::env::var_os("CODEX_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| dirs::home_dir().unwrap_or_default().join(".codex"))
}

pub fn app_data_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| dirs::home_dir().unwrap_or_default().join(".config"))
        .join("codex-companion")
}

pub fn portable_root() -> Option<PathBuf> {
    let executable = std::env::current_exe().ok()?;
    let root = executable.parent()?.to_path_buf();
    root.join("portable-mode").is_file().then_some(root)
}

pub fn config_dir(configuration: &AppConfiguration) -> PathBuf {
    if configuration.portable_mode {
        if let Some(root) = portable_root() {
            return root.join("config");
        }
    }
    app_data_dir().join("config")
}

pub fn backup_dir(configuration: &AppConfiguration) -> PathBuf {
    if configuration.portable_mode {
        if let Some(root) = portable_root() {
            return root.join("backups");
        }
    }
    app_data_dir().join("backups")
}

/// Personal bundles and recovered app packages remain Companion-owned, not in app roots.
pub fn personal_bundle_dir() -> PathBuf { app_data_dir().join("personal-bundles") }
pub fn personal_staging_dir() -> PathBuf { app_data_dir().join("staging") }

pub fn restore_history_file(configuration: &AppConfiguration) -> PathBuf {
    config_dir(configuration).join("restore-history-v1.json")
}

/// Safety archives for local legacy-session deletion live in Companion data, never in
/// CODEX_HOME. They are intentionally retained until the user removes them manually.
pub fn delete_quarantine_dir(configuration: &AppConfiguration) -> PathBuf {
    if configuration.portable_mode {
        if let Some(root) = portable_root() {
            return root.join("quarantine");
        }
    }
    app_data_dir().join("quarantine")
}
