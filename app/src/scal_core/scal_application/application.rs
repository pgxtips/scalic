use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::scal_core::scal_event::event_manager::EventManager;
use crate::scal_core::scal_system::ScalSystem;
use crate::scal_core::scal_application::configuration::ApplicationConfig;
use crate::scal_core::scal_window::ScalSDLWindow;

pub struct Application {
    pub win_handle: ScalSDLWindow,
    pub event_manager: EventManager,
    pub systems: Vec<Box<dyn ScalSystem>>,
    pub config: ApplicationConfig,
}

impl Application {
    pub fn run(&mut self) -> anyhow::Result<()> {
        loop {
            self.win_handle.canvas.clear();

            let events = self.win_handle.event_pump.poll_iter().collect::<Vec<_>>();


            for system in self.systems.iter_mut() {

                for events in events.iter() {
                    match events {
                        Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { 
                            std::process::exit(1);
                        },
                        _ => {}
                    }
                }

                system.handle_input_system(&mut self.config, &mut self.win_handle, &events);
                system.update_system(&mut self.config, &mut self.win_handle);
                system.run_system(&self.config, &mut self.win_handle);
            }

            self.win_handle.canvas.present();

            // sleep for 1/60th of a second (60fps)
            ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
