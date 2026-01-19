use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UIElement {
    pub name: String,
    pub control_type: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[cfg(target_os = "windows")]
pub mod implementation {
    use super::UIElement;
    use enigo::{Enigo, Key, KeyboardControllable, MouseControllable, Settings};
    use windows::{
        core::*,
        Win32::Foundation::*,
        Win32::System::Com::*,
        Win32::UI::Accessibility::*,
        Win32::UI::WindowsAndMessaging::*,
        Win32::System::Variant::*,
    };

    pub fn get_active_window_elements() -> Result<Vec<UIElement>> {
        let mut elements = Vec::new();

        unsafe {
            // Initialize COM
            CoInitializeEx(None, COINIT_MULTITHREADED).ok();

            // Create UIAutomation Instance
            let automation: IUIAutomation = CoCreateInstance(&CUIAutomation, None, CLSCTX_INPROC_SERVER)?;

            // Get Foreground Window
            let hwnd = GetForegroundWindow();
            if hwnd.0 == 0 {
                return Ok(elements);
            }

            // Get Element from Handle
            let root_element = automation.ElementFromHandle(hwnd)?;

            // Use ControlViewWalker to reduce noise (only actionable elements usually)
            // Or create a PropertyCondition. For now, we stick to FindAll with TrueCondition
            // but we rely on the `ControlView` logic implicitly if we wanted, but FindAll ignores Walker.

            // Optimization: Create a property condition for IsControlElement = true
            // UIA_IsControlElementPropertyId = 30016
            let is_control_prop = automation.CreatePropertyCondition(30016, Variant::from(true))?;

            // Using TreeScope_Descendants with IsControlElement=true is better than TrueCondition
            let element_array = root_element.FindAll(TreeScope_Descendants, &is_control_prop)?;
            let count = element_array.Length()?;

            for i in 0..count {
                let element = element_array.GetElement(i)?;

                // Get Name
                let name_bstr = element.CurrentName();
                let name = match name_bstr {
                    Ok(n) => n.to_string(),
                    Err(_) => "Unknown".to_string(),
                };

                // Get Control Type (ID)
                let control_type_id = element.CurrentControlType()?;
                let control_type = match control_type_id {
                    50000 => "Button",
                    50004 => "Edit",
                    50030 => "Document",
                    50005 => "Hyperlink",
                    50008 => "List",
                    50011 => "Menu",
                    50012 => "MenuBar",
                    50013 => "MenuItem",
                    _ => "Other",
                };

                if control_type == "Other" && name.is_empty() {
                    continue;
                }

                // Get Bounding Rectangle
                let rect = element.CurrentBoundingRectangle()?;

                elements.push(UIElement {
                    name,
                    control_type: control_type.to_string(),
                    x: rect.left,
                    y: rect.top,
                    width: rect.right - rect.left,
                    height: rect.bottom - rect.top,
                });
            }
        }

        Ok(elements)
    }

    pub fn perform_action(action_type: &str, x: i32, y: i32, text: Option<&str>) -> Result<()> {
        // Enigo 0.3.0+ requires Settings
        let mut enigo = Enigo::new(&Settings::default()).map_err(|e| anyhow::anyhow!("Failed to init Enigo: {:?}", e))?;

        match action_type {
            "click" => {
                enigo.mouse_move_to(x, y);
                enigo.mouse_click(enigo::MouseButton::Left);
            },
            "type" => {
                if let Some(t) = text {
                    enigo.mouse_move_to(x, y);
                    enigo.mouse_click(enigo::MouseButton::Left);
                    // Create a small delay might be needed in real world
                    std::thread::sleep(std::time::Duration::from_millis(50));
                    enigo.key_sequence(t);
                }
            },
            _ => {}
        }

        Ok(())
    }
}

#[cfg(not(target_os = "windows"))]
pub mod implementation {
    use super::UIElement;
    use anyhow::Result;

    pub fn get_active_window_elements() -> Result<Vec<UIElement>> {
        println!("Linux/Mac Mock: returning fake UI elements");
        Ok(vec![
            UIElement {
                name: "Mock Button".to_string(),
                control_type: "Button".to_string(),
                x: 100,
                y: 100,
                width: 50,
                height: 30,
            }
        ])
    }

    pub fn perform_action(action_type: &str, x: i32, y: i32, text: Option<&str>) -> Result<()> {
        println!("Linux/Mac Mock: Performing action {} at {},{} with text {:?}", action_type, x, y, text);
        Ok(())
    }
}
