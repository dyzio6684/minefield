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
    pub state: GameState,
}

#[derive(Clone, PartialEq)]
pub enum CellState {
    Hidden(u8),
    Flag(u8),
    Revealed(u8),
}

pub enum GameState {
    Playing,
    Lose,
    Win,
}