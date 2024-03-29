mod types;
mod events;
mod render;
mod widgets;
mod utils;

use std::{thread, time::Duration};

use events::run_event_loop;
use render::render;
use sdl2::{pixels::Color, hint, render::BlendMode};
use types::SdlData;
use utils::init_field;

fn init() -> SdlData {
    let sdl = sdl2::init().unwrap();

    hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let video = sdl.video().unwrap();
    let ttf = sdl2::ttf::init().unwrap();
    let window = video.window("Minefield", 240, 360)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas()
        .build()
        .unwrap();

    canvas.set_blend_mode(BlendMode::Blend);
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
    let mut data = init_field(10, 15, 12);

    loop {
        render(&mut sdl_data, &data);
        if !run_event_loop(&mut sdl_data, &mut data) {
            break;
        }

        thread::sleep(Duration::from_nanos(1_000_000_000 / 30));
    }
}
