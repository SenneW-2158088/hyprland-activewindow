use hyprland::event_listener::{EventListenerMutable as EventListener, WindowEventData};
use hyprland::Result;
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
struct CustomWindow {
    title: String,
    class: String,
}

impl From<WindowEventData> for CustomWindow {
    fn from(value: WindowEventData) -> Self {
        CustomWindow { title: value.window_title, class: value.window_class }
    }
}

fn main() -> Result<()> {
    
    // Create a event listener
    let mut event_listener = EventListener::new();
    event_listener.add_active_window_change_handler(move |event, _ | {
        let window_data = CustomWindow::from(event.expect("Failed to parse event data"));
        println!("{}", json!(window_data).to_string());
    });
    
    event_listener.start_listener()
}
