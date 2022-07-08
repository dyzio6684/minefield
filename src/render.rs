use sdl2::{image::LoadTexture, rect::Rect};

use crate::types::SdlData;

pub fn render(data: &mut SdlData) {
    data.canvas.clear();

    for widget in data.widgets.iter() {
        data.canvas.copy(
            &data.texture_creator.load_texture(widget.get_texture()).unwrap(),
            None,
            Some(Rect::new(
                widget.get_pos().0,
                widget.get_pos().1,
                widget.get_size().0,
                widget.get_size().1
            ))
        ).unwrap();
    }

    data.canvas.present();
}