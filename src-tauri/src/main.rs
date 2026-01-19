// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ingestion;
mod automation;
mod vision;

use ingestion::IngestionService;
use tauri::Manager;
use automation::implementation::{get_active_window_elements, perform_action};

#[tauri::command]
fn get_ui_tree() -> Result<Vec<automation::UIElement>, String> {
    get_active_window_elements().map_err(|e| e.to_string())
}

#[tauri::command]
fn execute_action(action_type: String, x: i32, y: i32, text: Option<String>) -> Result<(), String> {
    perform_action(&action_type, x, y, text.as_deref()).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Spawn the ingestion service in the background
            tauri::async_runtime::spawn(async move {
                let service = IngestionService::new();
                println!("Starting Ingestion Service...");
                service.run_loop().await;
            });

            // Initialize Vision
            vision::spawn_screenpipe();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_ui_tree, execute_action])
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
