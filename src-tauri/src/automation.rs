#[cfg(target_os = "windows")]
pub mod implementation {
    use anyhow::Result;
    use enigo::{Enigo, Settings, Mouse, Keyboard, Coordinate, Button, Direction};

    pub fn perform_action(action_type: &str, x: i32, y: i32, text: Option<&str>) -> Result<()> {
        let mut enigo = Enigo::new(&Settings::default()).map_err(|e| anyhow::anyhow!("Failed to init Enigo: {:?}", e))?;

        match action_type {
            "click" => {
                let _ = enigo.move_mouse(x, y, Coordinate::Abs);
                let _ = enigo.button(Button::Left, Direction::Click);
            },
            "type" => {
                if let Some(t) = text {
                    let _ = enigo.move_mouse(x, y, Coordinate::Abs);
                    let _ = enigo.button(Button::Left, Direction::Click);
                    // Create a small delay might be needed in real world
                    std::thread::sleep(std::time::Duration::from_millis(50));
                    let _ = enigo.text(t);
                }
            },
            _ => {}
        }

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
