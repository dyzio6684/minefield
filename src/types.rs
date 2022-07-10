use sdl2::{video::{Window, WindowContext}, render::{Canvas, TextureCreator}, EventPump, ttf::Sdl2TtfContext};

use crate::widgets::Cell;

pub struct SdlData {
    pub canvas: Canvas<Window>,
    pub ttf: Sdl2TtfContext,
    pub event_pump: EventPump,
    pub texture_creator: TextureCreator<WindowContext>,
}

pub struct GameData {
    pub cells: Vec<Cell>,
    pub width: usize,
    pub height: usize,
    pub mines: u32,
    pub state: GameState,
}

#[derive(Clone, PartialEq)]
pub enum CellState {
    Hidden(u8),
    Flag(u8),
    Revealed(u8),
}

pub enum GameState {
    Playing(u32),
    Lose,
    Win,
}