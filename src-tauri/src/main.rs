#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path;
use serde::{Deserialize, Serialize};
use tauri::{Manager, Runtime, Window};
use big_file_to_parts as BTF;
use big_file_to_parts::Options;

/// Тень от окна
pub fn set_shadow(
    window: impl raw_window_handle::HasRawWindowHandle,
    enable: bool,
) -> () {
	    match window.raw_window_handle() {

        raw_window_handle::RawWindowHandle::Win32(handle) => {
            use windows_sys::Win32::{
                Graphics::Dwm::DwmExtendFrameIntoClientArea, UI::Controls::MARGINS,
            };

            let m = if enable { 1 } else { 0 };
            let margins = MARGINS {
                cxLeftWidth: m,
                cxRightWidth: m,
                cyTopHeight: m,
                cyBottomHeight: m,
            };
            unsafe {
                DwmExtendFrameIntoClientArea(handle.hwnd as _, &margins);
            };
            ()
        }
        _ => return,
    }
}

/// Кодирование файла
#[tauri::command]
async fn encode_file<R: Runtime>(file_path: String, options: Options, window: Window<R>) -> Result<(), String> {

    let config = BTF::Config::new('e', &file_path, options);

    if let Err(e) = BTF::encode::encode_file(&config.path, config.options, window) {
        return Err(format!("{:?}", e));
    }

    Ok(())
}

/// Декодирование файла
#[tauri::command]
async fn decode_file(file_path: String, options: Options) -> Result<(), String> {

    let config = BTF::Config::new('d', &file_path, options);

    if let Err(e) = BTF::decode::decode_file(&config.path, config.options.path_for_save.unwrap_or(path::PathBuf::new())) {
        return Err(format!("{:?}", e));
    }

    Ok(())
}

#[tauri::command]
async fn get_me(options: Options) {
	println!("{:?}", options);
}

fn main() {
    tauri::Builder::default()
		.setup(|app| {
			//app.emit_all("test-event", get_me());
			let window = app.get_window("main").unwrap();
            set_shadow(&window, true);
            Ok(())
		})
        .invoke_handler(tauri::generate_handler![encode_file, decode_file, get_me])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
