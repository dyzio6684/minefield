use sdl2::{video::{Window, WindowContext}, render::{Canvas, TextureCreator}, EventPump};

use crate::widgets::Cell;

pub struct SdlData {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub texture_creator: TextureCreator<WindowContext>,
}

pub struct GameData {
    pub cells: Vec<Cell>,
    pub width: usize,
    pub height: usize,
}

#[derive(Clone, PartialEq)]
pub enum CellState {
    Hidden,
    Revealed(u8),
}