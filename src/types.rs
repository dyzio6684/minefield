use sdl2::{video::{Window, WindowContext}, render::{Canvas, TextureCreator}, EventPump};

use crate::widgets::Cell;

pub struct SdlData {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub texture_creator: TextureCreator<WindowContext>,
}

pub struct RenderData {
    pub cells: Vec<Cell>,
}