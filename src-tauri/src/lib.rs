use std::sync::Mutex;
use tauri::{Manager, RunEvent, WindowEvent};
use crate::commands::install::{add_install, game_launch, get_download_sizes, get_install_by_id, list_installs, list_installs_by_manifest_id, remove_install, update_install_dxvk_path, update_install_dxvk_version, update_install_env_vars, update_install_fps_value, update_install_game_path, update_install_launch_args, update_install_launch_cmd, update_install_pre_launch_cmd, update_install_prefix_path, update_install_runner_path, update_install_runner_version, update_install_skip_hash_valid, update_install_skip_version_updates, update_install_use_fps_unlock, update_install_use_jadeite, update_install_use_xxmi};
use crate::commands::manifest::{get_manifest_by_filename, get_manifest_by_id, list_game_manifests, get_game_manifest_by_filename, list_manifests_by_repository_id, update_manifest_enabled, get_game_manifest_by_manifest_id, list_compatibility_manifests, get_compatibility_manifest_by_manifest_id};
use crate::commands::repository::{list_repositories, remove_repository, add_repository, get_repository};
use crate::commands::settings::{block_telemetry_cmd, list_settings, update_extras, update_settings_default_fps_unlock_path, update_settings_default_game_path, update_settings_default_jadeite_path, update_settings_default_prefix_path, update_settings_default_xxmi_path, update_settings_launcher_action, update_settings_third_party_repo_updates};
use crate::utils::db_manager::{init_db, DbInstances};
use crate::utils::repo_manager::{load_manifests, ManifestLoader, ManifestLoaders, RunnerLoader};
use crate::utils::{block_telemetry, register_listeners, run_async_command, ActionBlocks};
use crate::utils::system_tray::init_tray;

mod utils;
mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .manage(ManifestLoaders {game: ManifestLoader::default(), runner: RunnerLoader::default()})
        .manage(Mutex::new(ActionBlocks { action_exit: false }))
        .setup(|app| {
            let handle = app.handle();
            run_async_command(async { init_db(&handle).await; });
            load_manifests(&handle);
            init_tray(&handle).unwrap();
            register_listeners(&handle);

            let path = app.path().app_data_dir().unwrap().join(".telemetry_blocked");
            if !path.exists() { block_telemetry(&handle);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![update_extras, block_telemetry_cmd, list_settings, update_settings_third_party_repo_updates, update_settings_default_game_path, update_settings_default_xxmi_path, update_settings_default_fps_unlock_path, update_settings_default_jadeite_path, update_settings_default_prefix_path, update_settings_launcher_action,
            remove_repository, add_repository, get_repository, list_repositories,
            get_manifest_by_id, get_manifest_by_filename, list_manifests_by_repository_id, update_manifest_enabled,
            get_game_manifest_by_filename, list_game_manifests, get_game_manifest_by_manifest_id,
            list_installs, list_installs_by_manifest_id, get_install_by_id, add_install, remove_install,
            update_install_game_path, update_install_runner_path, update_install_dxvk_path, update_install_skip_version_updates, update_install_skip_hash_valid, update_install_use_jadeite, update_install_use_xxmi, update_install_use_fps_unlock, update_install_fps_value, update_install_env_vars, update_install_pre_launch_cmd, update_install_launch_cmd, update_install_prefix_path, update_install_launch_args, update_install_dxvk_version, update_install_runner_version,
            list_compatibility_manifests, get_compatibility_manifest_by_manifest_id,
            game_launch, get_download_sizes])
        .build(tauri::generate_context!())
        .expect("Error while running KeqingLauncher!");

    builder.run(|app, event| {
        match &event {
            RunEvent::WindowEvent {event, ..} => {
                match event {
                    WindowEvent::CloseRequested { api, .. } => {
                        let blocks = app.state::<Mutex<ActionBlocks>>();
                        let state = blocks.lock().unwrap();

                        if state.action_exit {
                            app.get_window("main").unwrap().hide().unwrap();
                            api.prevent_close();
                        }
                    }
                    _ => {}
                }
            }
            RunEvent::Exit => {
                    run_async_command(async {
                        app.state::<DbInstances>().0.lock().await.get("db").unwrap().close().await;
                    });
            }
            _ => ()
        }
    })
}
