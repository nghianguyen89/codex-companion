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
