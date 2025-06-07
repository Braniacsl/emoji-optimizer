// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::{engine::general_purpose, Engine as _}; // Updated base64 import
use std::fs::File;
use std::io::Read;
use tauri::Manager; // Required for AppHandle, get_window

#[tauri::command]
async fn get_image_as_base64(path: String) -> Result<String, String> {
    let mut file = File::open(&path).map_err(|e| format!("Failed to open file: {}", e))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Use the general_purpose::STANDARD engine for base64 encoding
    let encoded_image = general_purpose::STANDARD.encode(&buffer);

    let mime_type = match std::path::Path::new(&path)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
    {
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        _ => "application/octet-stream", // Fallback
    };

    Ok(format!("data:{};base64,{}", mime_type, encoded_image))
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![get_image_as_base64])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
