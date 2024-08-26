
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::scal_buffer::Buffer;
use crate::scal_gfx::ScalSDLWindow;
use crate::scal_ui::ScalUI;

pub struct ScalInput;

impl ScalInput {
    pub fn handle_input(&self, win_handle: &mut ScalSDLWindow, scal_ui: &ScalUI) -> anyhow::Result<()>{

        let event_pump = &mut win_handle.event_pump;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { 
                    return Err(anyhow::Error::msg("Application Quit"));
                },
                Event::KeyDown { keycode, .. } => {
                    // check which current ui element has focus
                    // if buffer then add to buffer cells
                    // if command line then do command line action
                    log::info!("key pressed: {:?}", keycode);
                },
                Event::MouseButtonDown { x, y, .. } => {
                    // check which current ui element has focus
                    // if buffer then add to buffer cells
                    // if command line then do command line action
                    log::info!("mouse clicked at x: {}, y: {}", x, y);
                },
                _ => {}
            }
        }

        Ok(())
    }
}
