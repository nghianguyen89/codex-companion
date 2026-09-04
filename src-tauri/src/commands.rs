use crate::{
    backup, codex,
    config::{self, AppConfiguration, ConfigurationError},
    session_storage,
};
use tauri_plugin_dialog::DialogExt;

fn configuration_error(error: ConfigurationError) -> String {
    tracing::warn!("configuration error: {error}");
    error.to_string()
}
#[tauri::command]
pub fn get_diagnostics() -> Result<codex::DiagnosticsSnapshot, String> {
    let configuration = config::load().map_err(configuration_error)?;
    Ok(codex::diagnostics(&configuration))
}
#[tauri::command]
pub fn get_codex_paths() -> Result<codex::CodexPaths, String> {
    let configuration = config::load().map_err(configuration_error)?;
    Ok(codex::paths(&configuration))
}
#[tauri::command]
pub fn discover_conversations() -> session_storage::ConversationDiscovery {
    session_storage::discover_sessions()
}
#[tauri::command]
pub fn preview_backup(selected_ids: Vec<String>) -> Result<backup::BackupPreview, backup::BackupError> {
    let configuration = config::load().map_err(|error| backup::BackupError::from_message(configuration_error(error)))?;
    backup::preview(&configuration, &selected_ids).map_err(backup::BackupError::from_message)
}
#[tauri::command]
pub fn create_backup(selected_ids: Vec<String>) -> Result<backup::BackupResult, backup::BackupError> {
    let configuration = config::load().map_err(|error| backup::BackupError::from_message(configuration_error(error)))?;
    backup::create(&configuration, &selected_ids).map_err(backup::BackupError::from_message)
}
#[tauri::command]
pub fn inspect_backup_archive(
    app: tauri::AppHandle,
) -> Result<Option<backup::ArchiveInspection>, String> {
    let Some(selected) = app
        .dialog()
        .file()
        .set_title("Choose a Codex backup archive")
        .add_filter("ZIP archives", &["zip"])
        .blocking_pick_file()
    else {
        return Ok(None);
    };
    let path = selected
        .into_path()
        .map_err(|_| "The selected archive does not have a readable local path.")?;
    let mut inspection = backup::inspect(&path)?;
    if inspection.validation.valid {
        inspection.restore_token = Some(backup::register_restore_archive(path));
    }
    Ok(Some(inspection))
}
#[tauri::command]
pub fn preview_restore(restore_token: String, selected_ids: Vec<String>) -> Result<backup::RestorePreview, backup::BackupError> {
    let configuration = config::load().map_err(|error| backup::BackupError::from_message(configuration_error(error)))?;
    backup::preview_restore(&configuration, &restore_token, &selected_ids).map_err(backup::BackupError::from_message)
}
#[tauri::command]
pub fn restore_archive(restore_token: String, selected_ids: Vec<String>) -> Result<backup::RestoreResult, backup::BackupError> {
    let configuration = config::load().map_err(|error| backup::BackupError::from_message(configuration_error(error)))?;
    backup::restore(&configuration, &restore_token, &selected_ids).map_err(backup::BackupError::from_message)
}
#[tauri::command]
pub fn get_configuration() -> Result<AppConfiguration, String> {
    config::load().map_err(configuration_error)
}
#[tauri::command]
pub fn save_configuration(configuration: AppConfiguration) -> Result<(), String> {
    config::save(&configuration).map_err(configuration_error)
}
