mod types;
mod events;
mod render;
mod widgets;
mod utils;

use events::run_event_loop;
use render::render;
use sdl2::pixels::Color;
use types::{SdlData, GameData, GameState};
use utils::generate_cells;

fn init() -> SdlData {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let ttf = sdl2::ttf::init().unwrap();
    let window = video.window("Minefield", 240, 360)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas()
        .build()
        .unwrap();

    canvas.clear();
    canvas.set_draw_color(Color::RGB(31, 37, 47));
    canvas.present();

    let event_pump = sdl.event_pump().unwrap();
    let texture_creator = canvas.texture_creator();

    SdlData {
        canvas,
        ttf,
        event_pump,
        texture_creator,
    }
}

fn main() {
    let mut sdl_data = init();
    let mut data = GameData {
        cells: generate_cells(10, 15, 12),
        width: 10,
        height: 15,
        mines: 12,
        state: GameState::Playing(10 * 15),
    };

    loop {
        render(&mut sdl_data, &data);
        if !run_event_loop(&mut sdl_data, &mut data) {
            break;
        }
    }
}
