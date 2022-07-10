use crate::types::CellState;

use super::Widget;

pub const CELL_SIZE: u32 = 24;

#[derive(Clone)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub gx: usize,
    pub gy: usize,
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

    fn get_texture(&self) -> String {
        match self.state {
            CellState::Hidden => {
                "data/square9.png".to_string()
            }
            CellState::Revealed(i) => {
                format!("data/square{}.png", i)
            }
        }
    }

    fn mouse_down(&mut self) {
        
    }

    fn mouse_up(&self) {
        
    }
}