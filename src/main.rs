mod types;
mod events;
mod render;
mod widgets;

use events::run_event_loop;
use render::render;
use sdl2::pixels::Color;
use types::{SdlData, RenderData};
use widgets::Cell;

fn init() -> SdlData {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
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
        event_pump,
        texture_creator,
    }
}

fn generate_cells(w: usize, h: usize) -> Vec<Cell> {
    let mut temp = Vec::<Cell>::with_capacity(w * h);
    for x in 0..w {
        for y in 0..h {
            temp.push(Cell {
                x: (x * 24) as i32,
                y: (y * 24) as i32,
            });
        }
    }
    temp
}

fn main() {
    let mut sdl_data = init();
    let render_data = RenderData {
        cells: generate_cells(12, 16),
    };

    loop {
        render(&mut sdl_data, &render_data);
        if !run_event_loop(&mut sdl_data.event_pump) {
            break;
        }
    }
}
