mod backup;
mod codex;
mod commands;
mod config;
mod logging;
mod platform;
mod session_storage;

use commands::{
    create_backup, discover_conversations, get_codex_paths, get_configuration, get_diagnostics,
    inspect_backup_archive, preview_backup, save_configuration, preview_restore, restore_archive,
};

pub fn run() {
    logging::init();
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            get_diagnostics,
            get_codex_paths,
            discover_conversations,
            preview_backup,
            create_backup,
            inspect_backup_archive,
            preview_restore,
            restore_archive,
            get_configuration,
            save_configuration
        ])
        .run(tauri::generate_context!())
        .expect("error while running Codex Companion");
}
