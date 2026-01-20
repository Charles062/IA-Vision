#[cfg(target_os = "windows")]
pub mod implementation {
    use anyhow::Result;
    use enigo::{Enigo, Settings, Mouse, Keyboard, Coordinate, Button, Direction};

    pub fn perform_action(action_type: &str, x: i32, y: i32, text: Option<&str>) -> Result<()> {
        println!("ACTION: Received request to {} at ({}, {})", action_type, x, y);
        let mut enigo = Enigo::new(&Settings::default()).map_err(|e| {
            eprintln!("ACTION ERROR: Failed to init Enigo: {:?}", e);
            anyhow::anyhow!("Failed to init Enigo: {:?}", e)
        })?;

        match action_type {
            "click" => {
                println!("ACTION: Executing click...");
                if let Err(e) = enigo.move_mouse(x, y, Coordinate::Abs) {
                    eprintln!("ACTION ERROR: move_mouse failed: {:?}", e);
                }
                if let Err(e) = enigo.button(Button::Left, Direction::Click) {
                     eprintln!("ACTION ERROR: button click failed: {:?}", e);
                }
            },
            "type" => {
                if let Some(t) = text {
                    println!("ACTION: Executing type '{}'...", t);
                    if let Err(e) = enigo.move_mouse(x, y, Coordinate::Abs) {
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
                println!("ACTION WARNING: Unknown action type '{}'", action_type);
            }
        }

        println!("ACTION: Finished.");
        Ok(())
    }
}

#[cfg(not(target_os = "windows"))]
pub mod implementation {
    use anyhow::Result;

    pub fn perform_action(action_type: &str, x: i32, y: i32, text: Option<&str>) -> Result<()> {
        println!("Linux/Mac Mock: Performing action {} at {},{} with text {:?}", action_type, x, y, text);
        Ok(())
    }
}
