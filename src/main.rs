mod types;
mod events;
mod render;

use events::run_event_loop;
use render::render;
use sdl2::pixels::Color;
use types::SdlData;

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

fn main() {
    let mut sdl_data = init();

    loop {
        render(&mut sdl_data.canvas);
        if !run_event_loop(&mut sdl_data.event_pump) {
            break;
        }
    }
}
