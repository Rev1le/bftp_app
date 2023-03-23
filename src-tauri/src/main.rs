#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path;
use big_file_to_parts as BTF;
use big_file_to_parts::Options;

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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![encode_file, decode_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
