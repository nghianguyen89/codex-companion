mod backup;
mod beyond_compare;
mod fs_safety;
mod environment;
mod cleanup;
mod codex;
mod commands;
mod config;
mod logging;
mod local_delete;
mod platform;
mod personal_bundle;
mod restore_history;
mod session_storage;
mod sourcetree;

use commands::{
    create_backup, discover_conversations, get_codex_paths, get_configuration, get_diagnostics,
    inspect_backup_archive, preview_backup, save_configuration, preview_restore, restore_archive,
    get_restore_history, preview_local_delete, execute_local_delete,
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
            get_restore_history,
            preview_local_delete,
            execute_local_delete,
            get_configuration,
            save_configuration
            ,commands::preview_environment, commands::create_environment,
            commands::inspect_environment, commands::preview_environment_restore,
            commands::restore_environment, commands::scan_cleanup, commands::execute_cleanup
            ,commands::get_beyond_compare_readiness, commands::preview_beyond_compare,
            commands::create_beyond_compare_bundle, commands::inspect_beyond_compare_bundle,
            commands::preview_beyond_compare_recovery, commands::recover_beyond_compare
            ,commands::get_sourcetree_readiness, commands::preview_sourcetree,
            commands::create_sourcetree_bundle, commands::inspect_sourcetree_bundle,
            commands::preview_sourcetree_recovery, commands::recover_sourcetree
        ])
        .run(tauri::generate_context!())
        .expect("error while running Codex Companion");
}
