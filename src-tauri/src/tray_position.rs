use tauri::AppHandle;

#[cfg(target_os = "windows")]
mod platform {
    use super::*;
    use windows_sys::Win32::Foundation::RECT;
    use windows_sys::Win32::UI::WindowsAndMessaging::{SystemParametersInfoW, SPI_GETWORKAREA};

    pub fn compute(
        app: &AppHandle,
        cursor_x: f64,
        cursor_y: f64,
        width: f64,
        height: f64,
        gap: f64,
    ) -> (f64, f64) {
        let monitor = app.primary_monitor().unwrap().unwrap();
        let scale = monitor.scale_factor();

        let cx = cursor_x / scale;
        let cy = cursor_y / scale;

        let mut rect = RECT::default();
        unsafe {
            SystemParametersInfoW(SPI_GETWORKAREA, 0, &mut rect as *mut _ as _, 0);
        }

        let work_bottom = rect.bottom as f64 / scale;

        let x = cx - width / 2.0;
        let mut y = cy - height / 2.0;

        let max_y = work_bottom - height - gap;
        if y > max_y {
            y = max_y;
        }

        (x, y)
    }
}

#[cfg(target_os = "macos")]
mod platform {
    use super::*;

    pub fn compute(
        app: &AppHandle,
        cursor_x: f64,
        cursor_y: f64,
        width: f64,
        height: f64,
        gap: f64,
    ) -> (f64, f64) {
        let monitor = app.primary_monitor().unwrap().unwrap();
        let scale = monitor.scale_factor();
        let size = monitor.size();

        let cx = cursor_x / scale;

        // macOS menu bar is always at the top
        let menu_bar_height = 28.0;
        let work_top = menu_bar_height + gap;
        let work_bottom = size.height as f64 - gap;

        let mut x = cx - width / 2.0;
        let mut y = work_bottom - height;

        if y < work_top {
            y = work_top;
        }

        (x, y)
    }
}

#[cfg(target_os = "linux")]
mod platform {
    use super::*;

    pub fn compute(
        app: &AppHandle,
        cursor_x: f64,
        cursor_y: f64,
        width: f64,
        height: f64,
        gap: f64,
    ) -> (f64, f64) {
        let monitor = app.primary_monitor().unwrap().unwrap();
        let scale = monitor.scale_factor();
        let size = monitor.size();

        let cx = cursor_x / scale;

        // GNOME typically has a top panel (~32px)
        let top_panel = 32.0;
        let work_top = top_panel + gap;
        let work_bottom = size.height as f64 - gap;

        let mut x = cx - width / 2.0;
        let mut y = work_bottom - height;

        if y < work_top {
            y = work_top;
        }

        (x, y)
    }
}

pub fn initial_position(
    app: &AppHandle,
    cursor_x: f64,
    cursor_y: f64,
    width: f64,
    height: f64,
    gap: f64,
) -> (f64, f64) {
    platform::compute(app, cursor_x, cursor_y, width, height, gap)
}
