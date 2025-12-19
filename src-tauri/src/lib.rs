use tauri::{tray::TrayIconBuilder, utils::config_v1::WindowUrl, Manager, WebviewWindowBuilder};
use windows_sys::Win32::Foundation::RECT;
use windows_sys::Win32::UI::WindowsAndMessaging::{SystemParametersInfoW, SPI_GETWORKAREA};
mod tray_position;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            let icon = app.default_window_icon().unwrap().clone();
            let app_handle = app.handle().clone();
            let _tray = TrayIconBuilder::new()
                .icon(icon)
                .show_menu_on_left_click(false)
                .on_tray_icon_event(move |_tray_icon, event| match event {
                    tauri::tray::TrayIconEvent::Click { position, .. } => {
                        if let None = app_handle.get_webview_window("tray") {
                            // Window size
                            let width = 300.0;
                            let height = 300.0;

                            let (x, y) = tray_position::initial_position(
                                &app_handle,
                                position.x,
                                position.y,
                                width,
                                height,
                                20.0,
                            );

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
