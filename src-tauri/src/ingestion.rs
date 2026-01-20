use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;
use xcap::Monitor;
use active_win_pos_rs::get_active_window;
use image::DynamicImage;
use rusty_tesseract::{Args, Image};
use std::collections::HashMap;

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
        let img_clone = dynamic_image.clone();
        let ocr_text = tokio::task::spawn_blocking(move || perform_ocr(img_clone))
            .await
            .map_err(|e| anyhow!("OCR task join error: {}", e))?
            .map_err(|e| anyhow!("{}", e))?;

        Ok(ScreenData {
            timestamp: Utc::now(),
            app_name,
            window_title,
            ocr_text,
        })
    }
}

/// Realiza OCR em um buffer de imagem (PNG/JPEG bytes)
fn perform_ocr(img: DynamicImage) -> Result<String, String> {
    // 2. Prepara imagem para o Tesseract
    let tesseract_img = Image::from_dynamic_image(&img)
        .map_err(|e| format!("Erro ao converter imagem para Tesseract: {}", e))?;

    // 3. Configura argumentos (inglês padrão, whitelist de caracteres básicos)
    let args = Args {
        lang: "eng".to_string(),
        config_variables: HashMap::from([(
            "tessedit_char_whitelist".into(),
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.,:;?!@#$%&*()-_=+[]{}/\\\"'".into(),
        )]),
        ..Default::default()
    };

    // 4. Executa o OCR
    let text = rusty_tesseract::image_to_string(&tesseract_img, &args)
        .map_err(|e| format!("Falha na execução do Tesseract (Verifique se binário está instalado): {}", e))?;

    if text.trim().is_empty() {
        return Ok("[Nenhum texto detectado na tela]".to_string());
    }

    Ok(text)
}
