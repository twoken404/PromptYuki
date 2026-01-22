mod database;
mod commands;

use std::sync::Mutex;
use tauri::{Manager, Emitter, WindowEvent, menu::{Menu, MenuItem}, tray::{TrayIconBuilder, TrayIconEvent, MouseButton, MouseButtonState}};

#[derive(Default)]
struct AppState {
    pub minimize_to_tray: bool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::default();

    tauri::Builder::default()
        .manage(Mutex::new(app_state))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        // .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            commands::save_prompt,
            commands::delete_prompt,
            commands::get_all_prompts,
            commands::get_prompt_by_id,
            commands::increment_usage,
            commands::save_file,
            commands::set_tray_visible,
            commands::toggle_window_visibility,
            // commands::register_shortcut,
            // commands::unregister_shortcut,
            // commands::get_current_shortcut,
            commands::get_minimize_to_tray,
            commands::set_minimize_to_tray
        ])
        .setup(|app| {
            let app_handle = app.app_handle().clone();

            // 创建托盘菜单
            let show_item = MenuItem::with_id(app, "show", "显示界面", true, None::<&str>).unwrap();
            let quit_item = MenuItem::with_id(app, "quit", "退出程序", true, None::<&str>).unwrap();
            let menu = Menu::with_items(app, &[&show_item, &quit_item]).unwrap();

            // 创建托盘图标
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {
                        println!("menu item {:?} not handled", event.id);
                    }
                })
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.is_visible().map(|visible| {
                                if visible {
                                    let _ = window.hide();
                                } else {
                                    let _ = window.unminimize();
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                }
                            });
                        }
                    }
                    TrayIconEvent::Click {
                        button: MouseButton::Right,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        // 右键点击菜单会自动显示，不需要额外处理
                    }
                    TrayIconEvent::Move { .. } => {
                        // 鼠标移动事件，忽略
                    }
                    _ => {
                        println!("unhandled event {event:?}");
                    }
                })
                .build(app);

            // 注册全局快捷键并设置处理器
            // TODO: 临时注释掉，由于 capabilities 编译问题
            /*
            let window = app.get_webview_window("main").unwrap();
            let shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::Backquote);
            let shortcut_clone = shortcut.clone();

            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_handler(move |_app, scut, event| {
                        if scut == &shortcut_clone {
                            if let ShortcutState::Pressed = event.state() {
                                // 判断窗口是否可见，进行切换显示状态
                                if window.is_visible().unwrap() {
                                    let _ = window.hide(); // 隐藏窗口
                                } else {
                                    let _ = window.show(); // 显示窗口
                                    let _ = window.set_focus(); // 设置窗口焦点
                                }
                            }
                        }
                    })
                    .build(),
            )?;

            app_handle.global_shortcut().register(shortcut)?;
            */

            if let Some(window) = app.get_webview_window("main") {
                let window = window.clone();
                let app_handle = app_handle.clone();

                let win = window.clone();
                let _ = win.on_window_event(move |event| {
                    match event {
                        WindowEvent::CloseRequested { api, .. } => {
                            let state = app_handle.state::<Mutex<AppState>>();
                            let app_state = state.lock().unwrap();

                            if app_state.minimize_to_tray {
                                api.prevent_close();
                                let win2 = window.clone();
                                let _ = win2.hide();
                            }
                        }
                        _ => {}
                    }
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
