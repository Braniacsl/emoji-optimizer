use base64::{engine::general_purpose, Engine as _};
use image::{ImageFormat, RgbaImage};
use log::{error, info};
use std::io::Cursor;
use tauri::{AppHandle, Emitter as _, State};

use crate::state::AppState;

fn encode_and_emit(final_image: RgbaImage, app_handle: AppHandle) {
    info!("[Blocking task] Starting PNG and Base64 encoding...");

    let image_data: Result<Vec<u8>, _> = {
        let mut writer = Cursor::new(Vec::new());
        final_image
            .write_to(&mut writer, ImageFormat::Png)
            .map(|_| writer.into_inner())
    };

    let encoded_bytes = match image_data {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("[Blocking Task] Failed to encode image: {}", e);
            return;
        }
    };

    let base64_string = general_purpose::STANDARD.encode(encoded_bytes);
    let data_url = format!("data:image/png;base64,{}", base64_string);

    if let Err(e) = app_handle.emit("image-updated", Some(data_url)) {
        error!("[Blocking Task] Error emitting event: {}", e);
    } else {
        info!("[Blocking Task] Successfully emitted update to frontend.");
    }
}

#[tauri::command]
pub async fn open_image(
    state: State<'_, AppState>,
    app_handle: AppHandle,
    path: String,
) -> Result<(), String> {
    // Get image data from path
    let file = image::open(&path).map_err(|e| format!("Failed to open file: {}", e))?;

    let img: RgbaImage = file.to_rgba8();

    let rendered_image: RgbaImage;

    {
        let mut image_manager = state.image_manager.lock().await;

        image_manager
            .add_layer_from_image(img, path, Default::default())
            .map_err(|e| format!("Failed to open image: {}", e))?;

        rendered_image = image_manager.render();
    }

    tokio::spawn(async move {
        if let Err(e) =
            tokio::task::spawn_blocking(move || encode_and_emit(rendered_image, app_handle)).await
        {
            error!("Error in spawned blocking task: {}", e);
        }
    });

    Ok(())
}
