#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::process::Command;

fn main() {
  // Spawn local web server
  Command::new("npx")
  .arg("http-server")
  .arg("../dist")
  .spawn()
  .expect("failed to execute process");

  // Spawn tauri app
  let context = tauri::generate_context!();
  tauri::Builder::default()
    .menu(tauri::Menu::os_default(&context.package_info().name))
    .run(context)
    .expect("error while running tauri application");
}
