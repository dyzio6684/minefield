use sdl2::{image::LoadTexture, rect::Rect};

use crate::{types::{SdlData, GameData}, widgets::Widget};

pub fn render(sdl: &mut SdlData, data: &GameData) {
    sdl.canvas.clear();

    for widget in data.cells.iter() {
        sdl.canvas.copy(
            &sdl.texture_creator.load_texture(widget.get_texture()).unwrap(),
            None,
            Some(Rect::new(
                widget.get_pos().0,
                widget.get_pos().1,
                widget.get_size().0,
                widget.get_size().1
            ))
        ).unwrap();
    }

    sdl.canvas.present();
}