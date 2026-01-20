use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;
use xcap::Monitor;
use active_win_pos_rs::get_active_window;
use image::DynamicImage;
use rusty_tesseract::{Args, Image};

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
        // xcap 0.0.6 uses image 0.24, which we have aligned in Cargo.toml
        let dynamic_image = DynamicImage::ImageRgba8(image);

        // Get Active Window
        let (app_name, window_title) = match get_active_window() {
            Ok(window) => (window.app_name, window.title),
            Err(_) => ("Unknown".to_string(), "Unknown".to_string()),
        };

        // Perform OCR
        let ocr_text = self.perform_ocr(&dynamic_image).await?;

        Ok(ScreenData {
            timestamp: Utc::now(),
            app_name,
            window_title,
            ocr_text,
        })
    }

    async fn perform_ocr(&self, image: &DynamicImage) -> Result<String> {
        let image = image.clone();

        let text = tokio::task::spawn_blocking(move || {
            // Create Image from DynamicImage
            // rusty-tesseract 1.1+ supports from_dynamic_image.
            // Since we aligned image crate version to 0.24, this should work seamlessly.
            let img = Image::from_dynamic_image(&image)
                .map_err(|e| anyhow!("Failed to create Tesseract Image: {}", e))?;

            // Configure Tesseract arguments
            let args = Args {
                lang: "eng+por".to_string(),
                ..Args::default()
            };

            // Execute OCR
            rusty_tesseract::image_to_string(&img, &args)
                .map_err(|e| anyhow!("Tesseract OCR failed: {}", e))
        }).await.map_err(|e| anyhow!("OCR task join error: {}", e))??;

        Ok(text)
    }
}
