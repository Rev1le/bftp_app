#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use big_file_to_parts as BTF;
use big_file_to_parts::Options;

#[tauri::command]
async fn encode_file(file_path: String) -> Result<(), String> {
    let config = BTF::Config::new('e', &file_path);
    if let Err(e) = BTF::encode::encode_file(&config.path, config.options) {
        return Err(format!("{:?}", e));
    }
    Ok(())
}

#[tauri::command]
async fn decode_file(file_path: String) -> Result<(), String> {
    let config = BTF::Config::new('e', &file_path);
    if let Err(e) = BTF::decode::decode_file(&config.path) {
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
