// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ingestion;

use ingestion::IngestionService;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Spawn the ingestion service in the background
            tauri::async_runtime::spawn(async move {
                let service = IngestionService::new();
                println!("Starting Ingestion Service...");
                service.run_loop().await;
            });

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
