use crate::{
    backup, codex,
    config::{self, AppConfiguration, ConfigurationError},
    session_storage,
    local_delete,
};
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
pub fn preview_environment(groups: Vec<String>) -> Result<crate::environment::Preview, String> { crate::environment::preview(groups) }
#[tauri::command]
pub fn create_environment(token: String) -> Result<crate::environment::Created, String> { crate::environment::create(&token) }
#[tauri::command]
pub fn inspect_environment(app: tauri::AppHandle) -> Result<Option<crate::environment::Preview>, String> {
    let Some(file) = app.dialog().file().add_filter("Environment ZIP", &["zip"]).blocking_pick_file() else { return Ok(None); };
    crate::environment::inspect(file.into_path().map_err(|e| e.to_string())?).map(Some)
}
#[tauri::command]
pub fn preview_environment_restore(token: String) -> Result<crate::environment::RestorePreview, String> { crate::environment::preview_restore(&token) }
#[tauri::command]
pub fn restore_environment(token: String, confirmation: String) -> Result<crate::environment::Restored, String> { crate::environment::restore(&token, &confirmation) }
#[tauri::command]
pub fn scan_cleanup() -> Result<crate::cleanup::Scan, String> { crate::cleanup::scan() }
#[tauri::command]
pub fn execute_cleanup(token: String, confirmation: String) -> Result<crate::cleanup::Outcome, String> { crate::cleanup::execute(&token, &confirmation) }

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
pub fn get_restore_history() -> Result<Vec<crate::restore_history::RestoreHistoryEntry>, backup::BackupError> {
    let configuration = config::load().map_err(|error| backup::BackupError::from_message(configuration_error(error)))?;
    crate::restore_history::list(&configuration).map_err(backup::BackupError::from_message)
}
#[tauri::command]
pub fn preview_local_delete(selected_ids: Vec<String>) -> Result<local_delete::DeletePreview, backup::BackupError> {
    let configuration = config::load().map_err(|error| backup::BackupError::from_message(configuration_error(error)))?;
    local_delete::preview(&configuration, &selected_ids).map_err(backup::BackupError::from_message)
}
#[tauri::command]
pub fn execute_local_delete(selected_ids: Vec<String>, confirmation: String) -> Result<local_delete::DeleteResult, backup::BackupError> {
    let configuration = config::load().map_err(|error| backup::BackupError::from_message(configuration_error(error)))?;
    local_delete::execute(&configuration, &selected_ids, &confirmation).map_err(backup::BackupError::from_message)
}
#[tauri::command]
pub fn get_configuration() -> Result<AppConfiguration, String> {
    config::load().map_err(configuration_error)
}
#[tauri::command]
pub fn save_configuration(configuration: AppConfiguration) -> Result<(), String> {
    config::save(&configuration).map_err(configuration_error)
}
