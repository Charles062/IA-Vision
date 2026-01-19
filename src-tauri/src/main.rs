// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ingestion;
mod automation;
mod vision;
mod accessibility;

use ingestion::IngestionService;
use tauri::Manager;
use automation::implementation::perform_action;
use accessibility::implementation::get_ui_tree as get_ui_tree_impl;

#[tauri::command]
fn get_ui_tree() -> Result<Vec<accessibility::UIElement>, String> {
    get_ui_tree_impl().map_err(|e| e.to_string())
}

#[tauri::command]
fn execute_action(action_type: String, x: i32, y: i32, text: Option<String>) -> Result<(), String> {
    perform_action(&action_type, x, y, text.as_deref()).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            // Start Ollama with llama3 model
            println!("Starting Ollama server with llama3...");
            std::thread::spawn(|| {
                let _ = std::process::Command::new("ollama")
                    .args(["serve"])
                    .spawn();
                // Give the server a moment to start
                std::thread::sleep(std::time::Duration::from_secs(2));
                // Pull/run llama3 to ensure it's ready
                let _ = std::process::Command::new("ollama")
                    .args(["run", "llama3", "--keepalive", "24h"])
                    .spawn();
            });

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
