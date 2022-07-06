use sdl2::{video::{Window, WindowContext}, render::{Canvas, TextureCreator}, EventPump};

pub struct SdlData {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub texture_creator: TextureCreator<WindowContext>,
}