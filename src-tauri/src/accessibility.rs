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
pub mod implementation {
    use super::UIElement;
    use anyhow::Result;
    use windows::{
        core::*,
        Win32::Foundation::*,
        Win32::System::Com::*,
        Win32::UI::Accessibility::*,
        Win32::UI::WindowsAndMessaging::*,
        Win32::System::Variant::*,
    };

    pub fn get_ui_tree() -> Result<Vec<UIElement>> {
        unsafe {
            // Initialize COM safely
            // COINIT_MULTITHREADED is usually recommended for automation threads,
            // but Tauri/UI threads might need COINIT_APARTMENTTHREADED.
            // Using COINIT_MULTITHREADED as requested/safe default for worker threads.
            let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

            // Create UIAutomation Instance
            // If any step fails, we return empty list to trigger OCR fallback
            let automation: IUIAutomation = match CoCreateInstance(&CUIAutomation, None, CLSCTX_INPROC_SERVER) {
                Ok(a) => a,
                Err(_) => return Ok(Vec::new()),
            };

            // Get Foreground Window
            let hwnd = GetForegroundWindow();
            if hwnd.0.is_null() {
                return Ok(Vec::new());
            }

            // Get Element from Handle
            let root_element = match automation.ElementFromHandle(hwnd) {
                Ok(e) => e,
                Err(_) => return Ok(Vec::new()),
            };

            // Create TreeWalker (ControlViewWalker)
            // This walker filters out non-control elements (like raw layout containers sometimes)
            let walker = match automation.ControlViewWalker() {
                Ok(w) => w,
                Err(_) => return Ok(Vec::new()),
            };

            let mut elements = Vec::new();
            // Start recursive walk. Limit depth to avoid infinite loops or massive trees.
            // Depth 10 is usually enough for UI.
            // If walking fails partly, we still return what we found or just ignore errors.
            let _ = walk_tree(&root_element, &walker, 0, 10, &mut elements);

            Ok(elements)
        }
    }

    unsafe fn walk_tree(
        element: &IUIAutomationElement,
        walker: &IUIAutomationTreeWalker,
        depth: usize,
        max_depth: usize,
        results: &mut Vec<UIElement>,
    ) -> Result<()> {
        if depth > max_depth {
            return Ok(());
        }

        // Process current element (skip the root window itself if desired, but we include it usually)
        if let Ok(ui_element) = extract_element_info(element) {
            results.push(ui_element);
        }

        // Navigate to first child
        let mut child = walker.GetFirstChildElement(element);

        // Iterate through siblings
        while let Ok(current_child) = child {
            // Recursively walk the child
            walk_tree(&current_child, walker, depth + 1, max_depth, results)?;

            // Move to next sibling
            child = walker.GetNextSiblingElement(&current_child);
        }

        Ok(())
    }

    unsafe fn extract_element_info(element: &IUIAutomationElement) -> Result<UIElement> {
        // Name
        let name = element.CurrentName().unwrap_or_default().to_string();

        // Control Type
        let control_type_id = element.CurrentControlType().unwrap_or(UIA_ButtonControlTypeId); // Default or check
        let control_type = match control_type_id {
            UIA_ButtonControlTypeId => "Button",
            UIA_EditControlTypeId => "Edit",
            UIA_DocumentControlTypeId => "Document",
            UIA_HyperlinkControlTypeId => "Hyperlink",
            UIA_ListControlTypeId => "List",
            UIA_MenuControlTypeId => "Menu",
            UIA_MenuBarControlTypeId => "MenuBar",
            UIA_MenuItemControlTypeId => "MenuItem",
            UIA_WindowControlTypeId => "Window",
            UIA_TextControlTypeId => "Text",
            UIA_ComboBoxControlTypeId => "ComboBox",
            UIA_CheckBoxControlTypeId => "CheckBox",
            _ => "Other", // Simplified for now
        }.to_string();

        // Bounding Rectangle
        let rect = element.CurrentBoundingRectangle().unwrap_or(RECT { left: 0, top: 0, right: 0, bottom: 0 });
        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;

        // Is Enabled
        let is_enabled = element.CurrentIsEnabled().map(|b| b.as_bool()).unwrap_or(false);

        // Runtime ID (Safe array conversion can be tricky, fallback to AutomationID)
        let id = element.CurrentAutomationId().unwrap_or_default().to_string();
        // Ideally we use RuntimeID, but it returns a SAFEARRAY. AutomationID is easier string.
        // If AutomationID is empty, we could try RuntimeID, but for this task AutomationID is a good start.

        Ok(UIElement {
            id,
            name,
            control_type,
            bounding_box: [rect.left, rect.top, width, height],
            is_enabled,
        })
    }
}

#[cfg(not(target_os = "windows"))]
pub mod implementation {
    use super::UIElement;
    use anyhow::Result;

    pub fn get_ui_tree() -> Result<Vec<UIElement>> {
        println!("Linux/Mac Mock: returning fake UI tree");
        Ok(vec![
            UIElement {
                id: "mock_btn_1".to_string(),
                name: "Save".to_string(),
                control_type: "Button".to_string(),
                bounding_box: [100, 100, 80, 30],
                is_enabled: true,
            },
            UIElement {
                id: "mock_input_1".to_string(),
                name: "Username".to_string(),
                control_type: "Edit".to_string(),
                bounding_box: [100, 150, 200, 30],
                is_enabled: true,
            }
        ])
    }
}
