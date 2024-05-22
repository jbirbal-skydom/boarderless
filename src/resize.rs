use i_slint_backend_winit::winit::window::ResizeDirection;
use i_slint_backend_winit::WinitWindowAccessor;
use lazy_static::lazy_static;
use std::collections::HashMap;


extern crate lazy_static;

lazy_static! {
    static ref MAP: HashMap<String, ResizeDirection> = {
        let mut m = HashMap::new();
        m.insert("east".to_string(), ResizeDirection::East);
        m.insert("north".to_string(), ResizeDirection::North);
        m.insert("northeast".to_string(), ResizeDirection::NorthEast);
        m.insert("northwest".to_string(), ResizeDirection::NorthWest);
        m.insert("south".to_string(), ResizeDirection::South);
        m.insert("southeast".to_string(), ResizeDirection::SouthEast);
        m.insert("southwest".to_string(), ResizeDirection::SouthWest);
        m.insert("west".to_string(), ResizeDirection::West);
        m
    };
}



fn main() -> Result<(), slint::PlatformError> {
    let ui = ResizableWindow::new()?;
    let ui_weak = ui.as_weak();

    ui.on_resize(move |resize_direction_str| {
        let direction = MAP.get(&resize_direction_str.to_lowercase()).unwrap();

            let app_clone = ui_weak.unwrap();
            app_clone.window().with_winit_window(|win| {
                let _ = win.drag_resize_window(*direction);
            });
        
    });

    ui.run()
}

slint::slint!(import { ResizableWindow } from "./ui/resize.slint";);
