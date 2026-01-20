use super::UIElement;
use anyhow::{Result, anyhow};
use atspi::connection::AccessibilityConnection;
use atspi::{Accessible, Role};
use zbus::Connection;

// Helper to map Role
pub fn map_role(role: Role) -> String {
    match role {
        Role::PushButton => "Button".to_string(),
        Role::CheckMenu | Role::CheckMenuItem => "CheckBox".to_string(),
        Role::Menu | Role::MenuItem => "MenuItem".to_string(),
        Role::Window | Role::Frame => "Window".to_string(),
        Role::Text | Role::Entry | Role::Paragraph => "Text".to_string(),
        Role::Link => "Link".to_string(),
        _ => format!("{:?}", role),
    }
}

// Helper to get PID from a bus name using org.freedesktop.DBus interface
pub async fn get_pid_from_bus_name(conn: &Connection, name: &str) -> Result<u32> {
    let dbus_proxy = zbus::fdo::DBusProxy::new(conn).await?;
    let pid = dbus_proxy.get_connection_unix_process_id(name).await?;
    Ok(pid)
}

pub async fn get_active_window_tree(max_depth: u32) -> Result<Vec<UIElement>> {
    // 1. Get PID of active window
    let active_window = active_win_pos_rs::get_active_window()
        .map_err(|e| anyhow!("Failed to get active window: {}", e))?;

    // active-win-pos-rs returns process_id as u64, but DBus uses u32. Safe cast?
    let target_pid = active_window.process_id as u32;

    // 2. Connect to AT-SPI
    let a11y_conn = AccessibilityConnection::new().await
        .map_err(|e| anyhow!("Failed to connect to accessibility bus: {}", e))?;

    // 3. Find the Application with the matching PID
    let zbus_conn = a11y_conn.connection();

    // Use DBus proxy to list names
    let dbus_proxy = zbus::fdo::DBusProxy::new(zbus_conn).await?;
    let names = dbus_proxy.list_names().await?;

    let mut target_app_root: Option<Accessible> = None;

    for name in names {
        if !name.starts_with(':') {
            continue;
        }

        if let Ok(pid) = dbus_proxy.get_connection_unix_process_id(&name).await {
            if pid == target_pid {
                let accessible = Accessible {
                    name: name.to_string(),
                    path: "/org/a11y/atspi/accessible/root".into(),
                };
                target_app_root = Some(accessible);
                break;
            }
        }
    }

    let root = target_app_root.ok_or(anyhow!("Could not find accessibility application for PID {}", target_pid))?;

    // 4. Traverse the tree
    let mut elements = Vec::new();
    traverse_tree(&a11y_conn, root, 0, max_depth, &mut elements).await?;

    Ok(elements)
}

// Recursive traversal
#[async_recursion::async_recursion]
async fn traverse_tree(
    conn: &AccessibilityConnection,
    accessible: Accessible,
    depth: u32,
    max_depth: u32,
    results: &mut Vec<UIElement>
) -> Result<()> {
    if depth > max_depth {
        return Ok(());
    }

    let proxy = atspi::accessible::AccessibleProxy::builder(conn.connection())
        .destination(accessible.name.clone())?
        .path(accessible.path.clone())?
        .build()
        .await?;

    let name = proxy.name().await.unwrap_or_default();
    let role = proxy.get_role().await.unwrap_or(Role::Invalid);

    let states = proxy.get_state().await.unwrap_or_default();
    let is_enabled = states.contains(atspi::State::Enabled);

    let mut bbox = [0, 0, 0, 0];

    if let Ok(comp_proxy) = atspi::component::ComponentProxy::builder(conn.connection())
        .destination(accessible.name.clone())?
        .path(accessible.path.clone())?
        .build()
        .await
    {
        if let Ok(extents) = comp_proxy.get_extents(atspi::CoordType::Screen).await {
            bbox = [extents.x, extents.y, extents.width, extents.height];
        }
    }

    let id = format!("{}:{}", accessible.name, accessible.path.as_str());

    results.push(UIElement {
        id,
        name,
        control_type: map_role(role),
        bounding_box: bbox,
        is_enabled,
    });

    let child_count = proxy.child_count().await.unwrap_or(0);

    for i in 0..child_count {
        if let Ok(child_pair) = proxy.get_child_at_index(i).await {
             let (child_name, child_path) = child_pair;
             let child_accessible = Accessible {
                 name: child_name,
                 path: child_path.into(),
             };

             Box::pin(traverse_tree(conn, child_accessible, depth + 1, max_depth, results)).await?;
        }
    }

    Ok(())
}
