use super::Widget;

pub const CELL_SIZE: u32 = 24;

pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub mine: bool,
}

impl Widget for Cell {
    fn get_pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn get_size(&self) -> (u32, u32) {
        (CELL_SIZE, CELL_SIZE)
    }

    fn get_texture(&self) -> &str {
        if self.mine {
            "data/square9.png"
        } else {
            "data/square0.png"
        }
    }

    fn mouse_down(&self) {
        println!("{}", self.mine);
    }

    fn mouse_up(&self) {
        
    }
}