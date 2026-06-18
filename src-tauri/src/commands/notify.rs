use crate::error::{AppError, AppResult};
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use tauri::{path::BaseDirectory, AppHandle, Manager, Window};
#[cfg(not(target_os = "android"))]
use tauri::UserAttentionType;

const DEFAULT_SOUND_FILE: &str = "ding.wav";

#[tauri::command]
pub fn notify_system(title: String, body: String) -> AppResult<()> {
    notify_rust::Notification::new()
        .summary(&title)
        .body(&body)
        .show()
        .map_err(|err| AppError::Other(err.to_string()))?;
    Ok(())
}

#[tauri::command]
pub fn notify_sound(app: AppHandle, sound_file: String) -> AppResult<()> {
    let path = resolve_sound_path(&app, &sound_file)?;
    let (_stream, handle) =
        OutputStream::try_default().map_err(|err| AppError::Other(err.to_string()))?;
    let sink = Sink::try_new(&handle).map_err(|err| AppError::Other(err.to_string()))?;
    let file = File::open(path)?;
    let source =
        Decoder::new(BufReader::new(file)).map_err(|err| AppError::Other(err.to_string()))?;
    sink.append(source);
    sink.sleep_until_end();
    Ok(())
}

fn resolve_sound_path(app: &AppHandle, sound_file: &str) -> AppResult<PathBuf> {
    let requested = sound_file.trim();
    let requested = if requested.is_empty() {
        DEFAULT_SOUND_FILE
    } else {
        requested
    };
    let mut candidates = vec![requested.to_string()];
    if requested != DEFAULT_SOUND_FILE {
        candidates.push(DEFAULT_SOUND_FILE.to_string());
    }

    for candidate in candidates {
        let file_name = Path::new(&candidate)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(DEFAULT_SOUND_FILE);
        let path = app
            .path()
            .resolve(format!("assets/{file_name}"), BaseDirectory::Resource)
            .map_err(|err| AppError::Other(err.to_string()))?;
        if path.exists() {
            return Ok(path);
        }
    }

    Err(AppError::NotFound)
}

#[tauri::command]
pub fn notify_focus_window(window: Window) -> AppResult<()> {
    window
        .set_focus()
        .map_err(|err| AppError::Other(err.to_string()))?;
    Ok(())
}

#[tauri::command]
#[cfg(not(target_os = "android"))]
pub fn notify_taskbar_flash(window: Window) -> AppResult<()> {
    window
        .request_user_attention(Some(UserAttentionType::Informational))
        .map_err(|err| AppError::Other(err.to_string()))?;
    Ok(())
}

#[tauri::command]
#[cfg(target_os = "android")]
pub fn notify_taskbar_flash(_window: Window) -> AppResult<()> {
    Ok(())
}

#[tauri::command]
pub fn notify_vibration() -> AppResult<()> {
    Ok(())
}

#[tauri::command]
pub fn set_foreground_timer(
    _phase: String,
    _title: String,
    _body: String,
    _ends_at_ms: i64,
) -> AppResult<()> {
    Ok(())
}

#[tauri::command]
pub fn clear_foreground_timer() -> AppResult<()> {
    Ok(())
}
