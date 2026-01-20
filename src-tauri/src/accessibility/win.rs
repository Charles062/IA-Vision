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

pub fn get_ui_tree(max_depth_arg: u32) -> Result<Vec<UIElement>> {
    unsafe {
        // Initialize COM safely
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

        // Create UIAutomation Instance
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
        let walker = match automation.ControlViewWalker() {
            Ok(w) => w,
            Err(_) => return Ok(Vec::new()),
        };

        let mut elements = Vec::new();
        // Start recursive walk.
        let _ = walk_tree(&root_element, &walker, 0, max_depth_arg as usize, &mut elements);

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

    // Process current element
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
    let control_type_id = element.CurrentControlType().unwrap_or(UIA_ButtonControlTypeId);
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
        _ => "Other",
    }.to_string();

    // Bounding Rectangle
    let rect = element.CurrentBoundingRectangle().unwrap_or(RECT { left: 0, top: 0, right: 0, bottom: 0 });
    let width = rect.right - rect.left;
    let height = rect.bottom - rect.top;

    // Is Enabled
    let is_enabled = element.CurrentIsEnabled().map(|b| b.as_bool()).unwrap_or(false);

    // AutomationID
    let id = element.CurrentAutomationId().unwrap_or_default().to_string();

    Ok(UIElement {
        id,
        name,
        control_type,
        bounding_box: [rect.left, rect.top, width, height],
        is_enabled,
    })
}
