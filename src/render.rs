use sdl2::{image::LoadTexture, rect::Rect, pixels::Color};

use crate::{types::{SdlData, GameData, GameState}, widgets::Widget, utils::render_text};

pub fn render(sdl: &mut SdlData, data: &GameData) {
    sdl.canvas.set_draw_color(Color::RGB(31, 37, 47));
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
        GameState::Lose |
        GameState::Win => {
            sdl.canvas.set_draw_color(Color::RGBA(63, 63, 63, 127));
            sdl.canvas.fill_rect(Rect::new(4, 4, 200, 100)).unwrap();
        }
        _ => {}
    }

    match data.state {
        GameState::Lose => {
            render_text(sdl, "You lost!", Color::RGB(255, 63, 63), 8, 8);
            render_text(sdl, "Press 'N' to start", Color::WHITE, 8, 40);
            render_text(sdl, "a new game", Color::WHITE, 8, 70);
        }
        GameState::Win => {
            render_text(sdl, "You won!", Color::RGB(63, 255, 127), 8, 8);
            render_text(sdl, "Press 'N' to start", Color::WHITE, 8, 40);
            render_text(sdl, "a new game", Color::WHITE, 8, 70);
        }
        _ => {}
    }

    sdl.canvas.present();
}