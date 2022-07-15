use sdl2::{event::Event, mouse::MouseButton, keyboard::Keycode};

use crate::{types::{SdlData, GameData, CellState, GameState}, utils::{is_hovered, change_state, init_field}};

pub fn run_event_loop(sdl: &mut SdlData, data: &mut GameData) -> bool {
    for event in sdl.event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => return false,
            Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                match data.state {
                    GameState::Playing(_) => {
                        if mouse_btn == MouseButton::Left {
                            let cells = data.cells.clone();
                            for i in cells.iter() {
                                if is_hovered(i, x, y) {
                                    if let CellState::Hidden(_) = i.state {
                                        change_state(data, i.gx, i.gy, data.width, data.height, true);
                                    }
                                }
                            }
                            if let GameState::Playing(i) = data.state {
                                if i <= data.mines {
                                    data.state = GameState::Win;
                                }
                            }
                        } else if mouse_btn == MouseButton::Right {
                            for i in data.cells.iter_mut() {
                                if is_hovered(i, x, y) {
                                    if let CellState::Hidden(j) = i.state {
                                        i.state = CellState::Flag(j)
                                    } else if let CellState::Flag(j) = i.state {
                                        i.state = CellState::Hidden(j);
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            Event::KeyUp { keycode, .. } => {
                if let Some(k) = keycode {
                    if k == Keycode::N {
                        *data = init_field(10, 15, 12);
                    }
                }
            }
            _ => {}
        }
    }

    true
}