use base64::{encode, engine::general_purpose, Engine as _};
use image::{ImageFormat, RgbaImage};
use log::{error, info};
use std::{
    io::Cursor,
    sync::{Arc, Mutex},
};
use tauri::{AppHandle, Emitter as _, State};

use crate::{
    constants,
    state::{AppState, Layer},
};

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

    let final_image: RgbaImage;
    {
        let mut image_manager = state.image_manager.lock().await;

        image_manager
            .add_layer_from_image(img, path, Default::default())
            .map_err(|e| format!("Failed to open image: {}", e))?;

        final_image = image_manager.render()?;
    }

    tokio::spawn(async move {
        if let Err(e) =
            tokio::task::spawn_blocking(move || encode_and_emit(final_image, app_handle)).await
        {
            error!("Error in spawned blocking task: {}", e);
        }
    });

    Ok(())
}

#[tauri::command]
pub async fn toggle_grid(
    state: State<'_, AppState>,
    app_handle: AppHandle,
    active_layer: usize,
) -> Result<(), String> {
    info!("[toggle_grid]: toggled for layer {}", active_layer);

    let layer_mutex: Arc<Mutex<Layer>>;
    {
        let manager = state.image_manager.lock().await;
        let Some(mutex_clone) = manager.layers.get(active_layer).cloned() else {
            return Err(format!("Layer {} does not exist", active_layer));
        };

        layer_mutex = mutex_clone;
    }

    tokio::task::spawn_blocking(move || {
        let mut layer = layer_mutex.lock().unwrap();

        layer.grid.is_visible = !layer.grid.is_visible;

        layer.render();
    })
    .await
    .map_err(|e| format!("[toggle_grid] Error rendering layer: {}", e))?;

    let image_manager = state.image_manager.lock().await;

    let final_image = image_manager.render()?;

    encode_and_emit(final_image, app_handle);

    Ok(())
}
