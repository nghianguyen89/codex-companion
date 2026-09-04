use crate::{config::AppConfiguration, platform};
use serde::Serialize;
use std::{fs, path::Path, process::Command};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodexPaths {
    pub codex_home: String,
    pub config_dir: String,
    pub backup_dir: String,
    pub portable_mode: bool,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticsSnapshot {
    pub operating_system: String,
    pub architecture: String,
    pub codex_home: String,
    pub codex_home_exists: bool,
    pub config_dir: String,
    pub backup_dir: String,
    pub codex_cli_version: Option<String>,
    pub skills_count: usize,
    pub pets_count: usize,
}

pub fn paths(configuration: &AppConfiguration) -> CodexPaths {
    CodexPaths {
        codex_home: display(platform::codex_home()),
        config_dir: display(platform::config_dir(configuration)),
        backup_dir: display(platform::backup_dir(configuration)),
        portable_mode: configuration.portable_mode && platform::portable_root().is_some(),
    }
}
pub fn diagnostics(configuration: &AppConfiguration) -> DiagnosticsSnapshot {
    let paths = paths(configuration);
    let home = platform::codex_home();
    DiagnosticsSnapshot {
        operating_system: platform::operating_system().to_string(),
        architecture: std::env::consts::ARCH.to_string(),
        codex_home: paths.codex_home,
        codex_home_exists: home.is_dir(),
        config_dir: paths.config_dir,
        backup_dir: paths.backup_dir,
        codex_cli_version: cli_version(),
        skills_count: directory_entry_count(&home.join("skills")),
        pets_count: directory_entry_count(&home.join("pets")),
    }
}
fn display(path: impl AsRef<Path>) -> String {
    path.as_ref().to_string_lossy().to_string()
}
fn directory_entry_count(path: &Path) -> usize {
    fs::read_dir(path)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .filter(|entry| entry.path().is_dir())
                .count()
        })
        .unwrap_or(0)
}
pub(crate) fn cli_version() -> Option<String> {
    Command::new("codex")
        .arg("--version")
        .output()
        .ok()
        .filter(|result| result.status.success())
        .map(|result| String::from_utf8_lossy(&result.stdout).trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_directory_has_zero_entries() {
        assert_eq!(
            directory_entry_count(Path::new("not-a-real-codex-companion-path")),
            0
        );
    }
}
