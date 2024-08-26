use crate::scal_core::{scal_application::configuration::ApplicationConfig, scal_system::ScalSystem, scal_window::ScalSDLWindow};

pub struct EventSystem { }

impl EventSystem {
    pub fn new() -> Box<Self> {
        Box::new(EventSystem{})
    }
}

impl ScalSystem for EventSystem {
    fn run(&self, app_conf: &ApplicationConfig, win_handle: &mut ScalSDLWindow) {
    }
    fn update(&self, win_handle: &mut ScalSDLWindow) {
    }
    fn handle_input(&self, win_handle: &mut ScalSDLWindow) {

        let event_pump = &mut win_handle.event_pump;

        for events in event_pump.poll_iter() {
            match events {
                sdl2::event::Event::Quit {..} | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => { 
                   std::process::exit(1);
                },
                _ => {}
            }
        }

    }
} 
