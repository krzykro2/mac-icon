// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::menu::{IsMenuItem, MenuBuilder, MenuItemBuilder};
use tauri::tray::TrayIcon;
use tauri::{AppHandle, Manager, Runtime, Wry};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(move |app| {
            let tray = app.tray().unwrap();

            let show = MenuItemBuilder::with_id("show", "Open").build(app.handle());
            let mut menu_items: Vec<&dyn IsMenuItem<Wry>> = vec![&show];

            tray.set_menu(Some(
                MenuBuilder::new(app.handle())
                    .items(&menu_items[..])
                    .build()?,
            ))?;

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
