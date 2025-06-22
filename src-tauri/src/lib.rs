// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod image_manager;
mod layer;
mod state;

use commands::open_image;
use state::{AppState, ImageManager};
use tauri::async_runtime::Mutex;
use tauri::Manager;
use tauri_plugin_log::{Builder as LogBuilder, Target, TargetKind, TimezoneStrategy};

pub fn run() {
    let log_plugin = LogBuilder::new()
        .timezone_strategy(TimezoneStrategy::UseLocal)
        .targets([
            #[cfg(debug_assertions)]
            Target::new(TargetKind::Stdout),
            #[cfg(debug_assertions)]
            Target::new(TargetKind::Webview),
            #[cfg(not(debug_assertions))]
            Target::new(TargetKind::LogDir { file_name: None }),
        ])
        .build();

    tauri::Builder::default()
        .setup(|app| {
            let image_manager = ImageManager::new(512, 512);
            let state = Mutex::new(image_manager);

            app.manage(AppState {
                image_manager: state,
            });
            Ok(())
        })
        .plugin(log_plugin)
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![open_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
