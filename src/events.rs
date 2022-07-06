use sdl2::{EventPump, event::Event};

pub fn run_event_loop(event_pump: &mut EventPump) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => return false,
            _ => {}
        }
    }

    true
}