use super::Widget;

pub const CELL_SIZE: u32 = 24;

pub struct Cell {
    x: i32,
    y: i32,
}

impl Widget for Cell {
    fn get_pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn get_size(&self) -> (u32, u32) {
        (CELL_SIZE, CELL_SIZE)
    }

    fn get_texture(&self) -> &str {
        todo!()
    }
}