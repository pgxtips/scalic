use sdl2::{event::Event, keyboard::Keycode};

use crate::scal_core::{scal_application::configuration::ApplicationConfig, scal_system::ScalSystem, scal_window::ScalSDLWindow};

pub struct EventManager { }

impl EventManager {
    pub fn new() -> Self {
        EventManager{}
    }
}

impl EventManager {
    fn handle_events(&self, app_conf: &ApplicationConfig,  win_handle: &mut ScalSDLWindow) {
        let event_pump = &mut win_handle.event_pump;

        for events in event_pump.poll_iter() {
            match events {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { 
                   std::process::exit(1);
                },
                Event::KeyDown { .. } => {
                     println!("Key Down");
                },
                _ => {}
            }
        }

    }
} 
