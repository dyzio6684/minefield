use sdl2::{event::Event, mouse::MouseButton};

use crate::{types::{SdlData, GameData}, widgets::Widget, utils::is_hovered};

pub fn run_event_loop(sdl: &mut SdlData, data: &GameData) -> bool {
    for event in sdl.event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => return false,
            Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                if mouse_btn == MouseButton::Left {
                    for i in data.cells.iter() {
                        if is_hovered(i, x, y) {
                            i.mouse_down();
                        }
                    }
                }
            }
            _ => {}
        }
    }

    true
}