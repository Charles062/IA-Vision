use super::UIElement;
use anyhow::Result;

pub async fn get_active_window_tree(_max_depth: u32) -> Result<Vec<UIElement>> {
    // TEMPORARY STUB due to atspi build issues
    Ok(Vec::new())
}
