use sdl2::{render::Canvas, video::Window};

pub fn render(canvas: &mut Canvas<Window>) {
    canvas.clear();
    canvas.present();
}