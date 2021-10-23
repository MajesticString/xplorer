#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod drives;
mod files_api;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      files_api::read_directory,
      files_api::is_dir,
      drives::get_drives
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}