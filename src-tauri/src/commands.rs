use crate::{
    backup, codex,
    config::{self, AppConfiguration, ConfigurationError},
    session_storage,
    local_delete,
};
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
pub fn get_beyond_compare_readiness() -> crate::beyond_compare::Readiness { crate::beyond_compare::readiness() }
#[tauri::command]
pub fn preview_beyond_compare(app: tauri::AppHandle, credentials_included: bool) -> Result<Option<crate::personal_bundle::BundlePreview>, String> {
    let Some(file) = app.dialog().file().set_title("Choose a Beyond Compare settings package").add_filter("Beyond Compare package", &["bcpkg"]).blocking_pick_file() else { return Ok(None); };
    crate::personal_bundle::preview_create(file.into_path().map_err(|_| "The selected package does not have a readable local path.")?, credentials_included).map(Some)
}
#[tauri::command]
pub fn create_beyond_compare_bundle(token: String) -> Result<crate::personal_bundle::BundleCreated, String> { crate::personal_bundle::create(&token) }
#[tauri::command]
pub fn inspect_beyond_compare_bundle(app: tauri::AppHandle) -> Result<Option<crate::personal_bundle::BundleInspection>, String> {
    let Some(file) = app.dialog().file().set_title("Choose a personal bundle").add_filter("Personal bundle", &["zip"]).blocking_pick_file() else { return Ok(None); };
    crate::personal_bundle::inspect(file.into_path().map_err(|_| "The selected bundle does not have a readable local path.")?).map(Some)
}
#[tauri::command]
pub fn preview_beyond_compare_recovery(token: String) -> Result<crate::personal_bundle::RecoveryPreview, String> { crate::personal_bundle::preview_recovery(&token) }
#[tauri::command]
pub fn recover_beyond_compare(token: String, confirmation: String) -> Result<crate::personal_bundle::RecoveryResult, String> { crate::personal_bundle::recover(&token, &confirmation) }

#[tauri::command]
pub fn get_sourcetree_readiness() -> crate::sourcetree::Readiness { crate::sourcetree::readiness() }
#[tauri::command]
pub fn preview_sourcetree() -> Result<crate::sourcetree::Preview, String> { crate::sourcetree::preview() }
#[tauri::command]
pub fn create_sourcetree_bundle(token: String) -> Result<crate::sourcetree::Created, String> { crate::sourcetree::create(&token) }
#[tauri::command]
pub fn inspect_sourcetree_bundle(app: tauri::AppHandle) -> Result<Option<crate::sourcetree::Inspection>, String> {
    let Some(file) = app.dialog().file().set_title("Choose a SourceTree personal bundle").add_filter("Personal bundle", &["zip"]).blocking_pick_file() else { return Ok(None); };
    crate::sourcetree::inspect(file.into_path().map_err(|_| "The selected bundle does not have a readable local path.")?).map(Some)
}
#[tauri::command]
pub fn preview_sourcetree_recovery(token: String) -> Result<crate::sourcetree::RecoveryPreview, String> { crate::sourcetree::preview_recovery(&token) }
#[tauri::command]
pub fn recover_sourcetree(token: String, confirmation: String) -> Result<crate::sourcetree::RecoveryResult, String> { crate::sourcetree::recover(&token, &confirmation) }

#[tauri::command]
pub fn get_xampp_readiness() -> crate::xampp::Readiness { crate::xampp::readiness() }
#[tauri::command]
pub fn preview_xampp(app: tauri::AppHandle) -> Result<Option<crate::xampp::Preview>, String> {
    let Some(folders) = app.dialog().file().set_title("Select direct XAMPP htdocs project folders").blocking_pick_folders() else { return Ok(None); };
    let paths = folders.into_iter().map(|folder| folder.into_path().map_err(|_| "The selected XAMPP project does not have a readable local path.")).collect::<Result<Vec<_>, _>>()?;
    crate::xampp::preview(paths).map(Some)
}
#[tauri::command]
pub fn create_xampp_bundle(token: String) -> Result<crate::xampp::Created, String> { crate::xampp::create(&token) }
#[tauri::command]
pub fn inspect_xampp_bundle(app: tauri::AppHandle) -> Result<Option<crate::xampp::Inspection>, String> {
    let Some(file) = app.dialog().file().set_title("Choose an XAMPP personal bundle").add_filter("Personal bundle", &["zip"]).blocking_pick_file() else { return Ok(None); };
    crate::xampp::inspect(file.into_path().map_err(|_| "The selected bundle does not have a readable local path.")?).map(Some)
}
#[tauri::command]
pub fn preview_xampp_recovery(token: String) -> Result<crate::xampp::RecoveryPreview, String> { crate::xampp::preview_recovery(&token) }
#[tauri::command]
pub fn recover_xampp(token: String, confirmation: String) -> Result<crate::xampp::RecoveryResult, String> { crate::xampp::recover(&token, &confirmation) }

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
