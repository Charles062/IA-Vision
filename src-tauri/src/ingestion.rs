use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;
use xcap::Monitor;
use active_win_pos_rs::get_active_window;
use image::DynamicImage;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScreenData {
    pub timestamp: DateTime<Utc>,
    pub app_name: String,
    pub window_title: String,
    pub ocr_text: String,
}

pub struct IngestionService {
    // In a real implementation, we would hold the OCR engine instance here
    // engine: OcrEngine
}

impl IngestionService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run_loop(&self) {
        loop {
            match self.capture_context().await {
                Ok(data) => {
                    println!("Captured: [{}] {} - Text len: {}",
                        data.timestamp,
                        data.app_name,
                        data.ocr_text.len()
                    );
                    // TODO: Save to SQLite Vector DB
                },
                Err(e) => {
                    eprintln!("Error capturing context: {}", e);
                }
            }
            sleep(Duration::from_secs(5)).await;
        }
    }

    async fn capture_context(&self) -> Result<ScreenData> {
        let monitors = Monitor::all().map_err(|e| anyhow!("Failed to get monitors: {}", e))?;
        let monitor = monitors.first().ok_or(anyhow!("No monitor found"))?;

        // Capture screenshot
        let image = monitor.capture_image().map_err(|e| anyhow!("Screenshot failed: {}", e))?;

        // Convert to DynamicImage for processing if needed, though xcap returns RgbaImage usually
        let dynamic_image = DynamicImage::ImageRgba8(image);

        // Get Active Window
        let (app_name, window_title) = match get_active_window() {
            Ok(window) => (window.app_name, window.title),
            Err(_) => ("Unknown".to_string(), "Unknown".to_string()),
        };

        // Perform OCR
        let ocr_text = self.perform_ocr(&dynamic_image)?;

        Ok(ScreenData {
            timestamp: Utc::now(),
            app_name,
            window_title,
            ocr_text,
        })
    }

    fn perform_ocr(&self, image: &DynamicImage) -> Result<String> {
        // NOTE: In a production environment with `ocrs`, we would:
        // 1. Prepare the image (resize, grayscale).
        // 2. Pass it to the loaded OCR engine.
        // 3. Extract text.

        // For now, return a placeholder to verify the pipeline structure
        Ok("OCR Integration Point: Real text would appear here.".to_string())
    }
}
