#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{SystemTray, SystemTrayEvent, Manager};
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let quit = CustomMenuItem::new("quit".to_string(), "Exit");
    // let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let show = CustomMenuItem::new("show".to_string(), "Open Screen Blackout");
    let blackout = CustomMenuItem::new("blackout".to_string(), "Start Blackout");
    let stop_blackout = CustomMenuItem::new("stop-blackout".to_string(), "Stop Blackout");
    let tray_menu = SystemTrayMenu::new()
        .add_item(blackout)
        .add_item(stop_blackout)
        .add_item(show)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
              match id.as_str() {
                "blackout" => {
                    _ = app.emit_to("main", "blackout", {});
                }
                "stop-blackout" => {
                  _  = app.emit_to("main", "stop-blackout", {});
                },
                "quit" => {
                  std::process::exit(0);
                }
                "hide" => {
                  let window = app.get_window("main").unwrap();
                  _ = window.hide();
                }
                "show" => {
                    let window = app.get_window("main").unwrap();
                    _ = window.show();
                  }
                _ => {}
              }
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
              api.prevent_exit();
            }
            _ => {}
          });
        // .expect("error while running tauri application");



}
