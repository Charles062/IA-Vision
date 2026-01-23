use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Action {
    #[serde(rename = "action")]
    pub action_type: String,
    #[serde(default)]
    pub x: i32,
    #[serde(default)]
    pub y: i32,
    pub text: Option<String>,
}

#[cfg(target_os = "windows")]
pub mod implementation {
    use anyhow::Result;
    use enigo::{Enigo, Settings, Mouse, Keyboard, Coordinate, Button, Direction};
    use super::Action;

    pub fn perform_actions(actions: Vec<Action>) -> Result<()> {
        let mut enigo = Enigo::new(&Settings::default()).map_err(|e| {
            eprintln!("ACTION ERROR: Failed to init Enigo: {:?}", e);
            anyhow::anyhow!("Failed to init Enigo: {:?}", e)
        })?;

        for action in actions {
            println!("ACTION: Received request to {} at ({}, {})", action.action_type, action.x, action.y);
            match action.action_type.as_str() {
                "click" => {
                    println!("ACTION: Executing click...");
                    if let Err(e) = enigo.move_mouse(action.x, action.y, Coordinate::Abs) {
                        eprintln!("ACTION ERROR: move_mouse failed: {:?}", e);
                    }
                    if let Err(e) = enigo.button(Button::Left, Direction::Click) {
                         eprintln!("ACTION ERROR: button click failed: {:?}", e);
                    }
                },
                "type" => {
                    if let Some(t) = &action.text {
                        println!("ACTION: Executing type '{}'...", t);
                        if let Err(e) = enigo.move_mouse(action.x, action.y, Coordinate::Abs) {
                            eprintln!("ACTION ERROR: move_mouse failed: {:?}", e);
                        }
                        if let Err(e) = enigo.button(Button::Left, Direction::Click) {
                             eprintln!("ACTION ERROR: button click failed: {:?}", e);
                        }
                        std::thread::sleep(std::time::Duration::from_millis(50));
                        if let Err(e) = enigo.text(t) {
                            eprintln!("ACTION ERROR: text input failed: {:?}", e);
                        }
                    }
                },
                _ => {
                    println!("ACTION WARNING: Unknown action type '{}'", action.action_type);
                }
            }
            // Small delay between actions to allow UI to react
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        println!("ACTION: Finished batch.");
        Ok(())
    }

    pub fn perform_action(action_type: &str, x: i32, y: i32, text: Option<&str>) -> Result<()> {
       perform_actions(vec![Action {
           action_type: action_type.to_string(),
           x,
           y,
           text: text.map(|s| s.to_string()),
       }])
    }
}

#[cfg(not(target_os = "windows"))]
pub mod implementation {
    use anyhow::Result;
    use super::Action;

    pub fn perform_actions(actions: Vec<Action>) -> Result<()> {
        println!("Linux/Mac Mock: Performing batch of {} actions", actions.len());
        for action in actions {
            println!(" - {} at {},{}", action.action_type, action.x, action.y);
        }
        Ok(())
    }

    pub fn perform_action(action_type: &str, x: i32, y: i32, text: Option<&str>) -> Result<()> {
        println!("Linux/Mac Mock: Performing action {} at {},{} with text {:?}", action_type, x, y, text);
        Ok(())
    }
}
