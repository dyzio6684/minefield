use crate::types::CellState;

use super::Widget;

pub const CELL_SIZE: u32 = 24;

pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub mine: bool,
    pub state: CellState,
}

impl Widget for Cell {
    fn get_pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn get_size(&self) -> (u32, u32) {
        (CELL_SIZE, CELL_SIZE)
    }

    fn get_texture(&self) -> &str {
        match self.state {
            CellState::Hidden => {
                "data/square9.png"
            }
            CellState::Revealed(_) => {
                if self.mine {
                    "data/square10.png"
                } else {
                    "data/square0.png"
                }
            }
        }
    }

    fn mouse_down(&mut self) {
        self.state = CellState::Revealed(0);
    }

    fn mouse_up(&self) {
        
    }
}