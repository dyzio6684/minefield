use sdl2::{image::LoadTexture, rect::Rect, pixels::Color};

use crate::{types::{SdlData, GameData, GameState}, widgets::Widget};

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

    match data.state {
        GameState::Lose => {
            let surface = sdl.ttf.load_font("data/Lato-Regular.ttf", 24)
                .unwrap()
                .render("You lost")
                .solid(Color::RGB(255, 127, 127))
                .unwrap();
            let texture = sdl.texture_creator.create_texture_from_surface(surface).unwrap();
            sdl.canvas.copy(&texture, None, Rect::new(8, 8, 80, 24)).unwrap();
        }
        GameState::Win => {
            let surface = sdl.ttf.load_font("data/Lato-Regular.ttf", 24)
                .unwrap()
                .render("You won")
                .solid(Color::RGB(127, 255, 127))
                .unwrap();
            let texture = sdl.texture_creator.create_texture_from_surface(surface).unwrap();
            sdl.canvas.copy(&texture, None, Rect::new(8, 8, 80, 24)).unwrap();
        }
        _ => {}
    }

    sdl.canvas.present();
}