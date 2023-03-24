#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path;
use tauri::Manager;
use big_file_to_parts as BTF;
use big_file_to_parts::Options;

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

#[tauri::command]
async fn encode_file(file_path: String, path_for_save: String) -> Result<(), String> {
    let option = Options {
        path_for_save: Some(path::PathBuf::from(path_for_save)),
        ..Options::default()
    };
    let config = BTF::Config::new('e', &file_path, option);

    if let Err(e) = BTF::encode::encode_file(&config.path, config.options) {
        return Err(format!("{:?}", e));
    }
    Ok(())
}

#[tauri::command]
async fn decode_file(file_path: String, path_for_save: String) -> Result<(), String> {
    let option = Options {
        path_for_save: Some(path::PathBuf::from(path_for_save)),
        ..Options::default()
    };
    let config = BTF::Config::new('e', &file_path, option);

    if let Err(e) = BTF::decode::decode_file(&config.path, config.options.path_for_save.unwrap_or(path::PathBuf::new())) {
        return Err(format!("{:?}", e));
    }
    Ok(())
}

pub fn get_me() -> String {
	String::from("Hi Man")
}

fn main() {
    tauri::Builder::default()
		.setup(|app| {
			//app.emit_all("test-event", get_me());
			let window = app.get_window("main").unwrap();
            set_shadow(&window, true);
            Ok(())
		})
        .invoke_handler(tauri::generate_handler![encode_file, decode_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
