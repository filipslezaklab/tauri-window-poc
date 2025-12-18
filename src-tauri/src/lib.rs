use std::ops::Deref;

use tauri::{tray::TrayIconBuilder, utils::config_v1::WindowUrl, Manager, WebviewWindowBuilder};
use windows_sys::Win32::Foundation::RECT;
use windows_sys::Win32::UI::WindowsAndMessaging::{SystemParametersInfoW, SPI_GETWORKAREA};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let icon = app.default_window_icon().unwrap().clone();
            let app_handle = app.handle().clone();
            let tray = TrayIconBuilder::new()
                .icon(icon)
                .show_menu_on_left_click(false)
                .on_tray_icon_event(move |tray_icon, event| match event {
                    tauri::tray::TrayIconEvent::Click {
                        id,
                        position,
                        rect,
                        button,
                        button_state,
                    } => {
                        if let None = app_handle.get_webview_window("tray") {
                            // Window size
                            let width = 300.0;
                            let height = 300.0;

                            // Get primary monitor scale factor
                            let monitor = app_handle.primary_monitor().unwrap().unwrap();
                            let scale = monitor.scale_factor();

                            // Convert cursor position to logical coords
                            let cursor_x = position.x / scale;
                            let cursor_y = position.y / scale;

                            // Work area (physical → logical)
                            let mut rect = RECT::default();
                            unsafe {
                                SystemParametersInfoW(
                                    SPI_GETWORKAREA,
                                    0,
                                    &mut rect as *mut _ as _,
                                    0,
                                );
                            }

                            let work_bottom = rect.bottom as f64 / scale;

                            // Center window on cursor
                            let mut x = cursor_x - width / 2.0;
                            let mut y = cursor_y - height / 2.0;

                            // Clamp above taskbar
                            let max_y = work_bottom - height - 20.0;
                            if y > max_y {
                                y = max_y;
                            }

                            WebviewWindowBuilder::new(
                                &app_handle,
                                "tray",
                                tauri::WebviewUrl::App("tray".into()),
                            )
                            .title("Tray window")
                            .inner_size(width, height)
                            .position(x, y)
                            .resizable(false)
                            .decorations(false)
                            .always_on_top(true)
                            .transparent(true)
                            .skip_taskbar(true)
                            .build()
                            .unwrap();
                        }
                    }
                    _ => {
                        println!("Unhandled tray event {event:?}");
                    }
                })
                .build(app)
                .unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
