use crate::scal_core::scal_system::ScalSystem;
use crate::scal_core::scal_application::configuration::ApplicationConfig;
use crate::scal_core::scal_window::ScalSDLWindow;

pub struct Application {
    pub win_handle: ScalSDLWindow,
    pub systems: Vec<Box<dyn ScalSystem>>,
    pub config: ApplicationConfig,
}

impl Application {
    pub fn run(&mut self) -> anyhow::Result<()> {
        loop {
            self.win_handle.canvas.clear();

            for system in self.systems.iter() {
                system.handle_input(&mut self.win_handle);
                system.update(&mut self.win_handle);
                system.run(&self.config, &mut self.win_handle);
            }

            self.win_handle.canvas.present();

            // sleep for 1/60th of a second (60fps)
            ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
