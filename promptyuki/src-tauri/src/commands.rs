use crate::database::{get_database_path, Prompt};
use crate::AppState;
use serde_json::Value;
use tauri::{command, AppHandle, Manager, State};

use std::fs;
use std::path::Path;
use std::sync::Mutex;

#[command]
pub async fn save_prompt(prompt: Prompt) -> Result<Prompt, String> {
    let db_path = get_database_path();

    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let prompts = load_prompts(&db_path).unwrap_or_default();
    let _prompts_json: Value = serde_json::from_value(
        serde_json::to_value(&prompts).unwrap_or(Value::Array(vec![]))
    ).unwrap_or(Value::Array(vec![]));

    Ok(prompt)
}

#[command]
pub async fn delete_prompt(_id: String) -> Result<(), String> {
    Ok(())
}

#[command]
pub async fn get_all_prompts() -> Result<Vec<Prompt>, String> {
    Ok(vec![])
}

#[command]
pub async fn get_prompt_by_id(_id: String) -> Result<Option<Prompt>, String> {
    Ok(None)
}

#[command]
pub async fn increment_usage(_id: String) -> Result<(), String> {
    Ok(())
}

#[command]
pub async fn save_file(content: String, _default_filename: String) -> Result<String, String> {
    Ok(content)
}

#[command]
pub async fn toggle_window_visibility(app_handle: AppHandle) -> Result<bool, String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        let is_visible = window.is_visible().map_err(|e| e.to_string())?;
        if is_visible {
            window.hide().map_err(|e| e.to_string())?;
            Ok(false)
        } else {
            window.show().map_err(|e| e.to_string())?;
            window.set_focus().map_err(|e| e.to_string())?;
            Ok(true)
        }
    } else {
        Err("Window not found".to_string())
    }
}

#[command]
pub async fn set_tray_visible(_app_handle: AppHandle, _visible: bool) -> Result<(), String> {
    Ok(())
}

// 暂时注释掉快捷键相关功能
/*
#[command]
pub async fn register_shortcut(app_handle: AppHandle, shortcut: String) -> Result<(), String> {
    app_handle
        .global_shortcut()
        .unregister_all()
        .map_err(|e| e.to_string())?;

    let (mods, code) = parse_shortcut_string(&shortcut);
    app_handle
        .global_shortcut()
        .register(Shortcut::new(mods, code))
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn parse_shortcut_string(shortcut: &str) -> (Option<Modifiers>, Code) {
    let parts: Vec<&str> = shortcut.split('+').collect();
    let mut mods = Modifiers::empty();
    let mut code = Code::KeyY;

    for part in &parts {
        match *part {
            "Ctrl" | "Control" => mods |= Modifiers::CONTROL,
            "Alt" => mods |= Modifiers::ALT,
            "Shift" => mods |= Modifiers::SHIFT,
            "Meta" | "Win" => mods |= Modifiers::META,
            _ => {
                code = parse_key_code(part);
            }
        }
    }

    (Some(mods), code)
}

fn parse_key_code(key: &str) -> Code {
    match key.to_uppercase().as_str() {
        "A" => Code::KeyA,
        "B" => Code::KeyB,
        "C" => Code::KeyC,
        "D" => Code::KeyD,
        "E" => Code::KeyE,
        "F" => Code::KeyF,
        "G" => Code::KeyG,
        "H" => Code::KeyH,
        "I" => Code::KeyI,
        "J" => Code::KeyJ,
        "K" => Code::KeyK,
        "L" => Code::KeyL,
        "M" => Code::KeyM,
        "N" => Code::KeyN,
        "O" => Code::KeyO,
        "P" => Code::KeyP,
        "Q" => Code::KeyQ,
        "R" => Code::KeyR,
        "S" => Code::KeyS,
        "T" => Code::KeyT,
        "U" => Code::KeyU,
        "V" => Code::KeyV,
        "W" => Code::KeyW,
        "X" => Code::KeyX,
        "Y" => Code::KeyY,
        "Z" => Code::KeyZ,
        "0" => Code::Digit0,
        "1" => Code::Digit1,
        "2" => Code::Digit2,
        "3" => Code::Digit3,
        "4" => Code::Digit4,
        "5" => Code::Digit5,
        "6" => Code::Digit6,
        "7" => Code::Digit7,
        "8" => Code::Digit8,
        "9" => Code::Digit9,
        "ENTER" => Code::Enter,
        "ESCAPE" => Code::Escape,
        "BACKSPACE" => Code::Backspace,
        "TAB" => Code::Tab,
        "SPACE" => Code::Space,
        "BACKQUOTE" | "`" => Code::Backquote,
        _ => Code::Backquote,
    }
}

#[command]
pub async fn unregister_shortcut(app_handle: AppHandle) -> Result<(), String> {
    app_handle
        .global_shortcut()
        .unregister_all()
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[command]
pub async fn get_current_shortcut(_app_handle: AppHandle) -> Result<String, String> {
    Ok("Alt".to_string())
}
*/

#[command]
pub async fn get_minimize_to_tray(state: State<'_, Mutex<AppState>>) -> Result<bool, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    Ok(app_state.minimize_to_tray)
}

#[command]
pub async fn set_minimize_to_tray(state: State<'_, Mutex<AppState>>, minimize: bool) -> Result<(), String> {
    let mut app_state = state.lock().map_err(|e| e.to_string())?;
    app_state.minimize_to_tray = minimize;
    Ok(())
}

fn load_prompts(path: &Path) -> Result<Vec<Prompt>, String> {
    if path.exists() {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).map_err(|e| e.to_string())
    } else {
        Ok(vec![])
    }
}
