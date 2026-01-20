use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UIElement {
    pub id: String,       // RuntimeID or AutomationID
    pub name: String,     // The text label (e.g., "Save")
    pub control_type: String, // e.g., "Button", "Edit", "Document"
    pub bounding_box: [i32; 4], // [x, y, width, height]
    pub is_enabled: bool,
}

#[cfg(target_os = "windows")]
mod win;

#[cfg(target_os = "linux")]
mod linux;

pub async fn get_ui_tree(max_depth: u32) -> Result<Vec<UIElement>, String> {
    #[cfg(target_os = "windows")]
    {
        // Offload to a blocking thread because Windows COM is blocking
        tokio::task::spawn_blocking(move || {
            win::get_ui_tree(max_depth).map_err(|e| e.to_string())
        })
        .await
        .map_err(|e| e.to_string())?
    }

    #[cfg(target_os = "linux")]
    {
        linux::get_active_window_tree(max_depth).await.map_err(|e| e.to_string())
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        // Fallback for other OSs (like macOS during dev) or just error
        println!("Unsupported OS or Mocking for non-Linux/Windows");
        Ok(vec![])
    }
}
