use sdl2::event::Event;

use crate::scal_core::{scal_application::configuration::ApplicationConfig, scal_buffer::buffer::Buffer, scal_system::ScalSystem, scal_window::ScalSDLWindow};

pub struct BufferSystem {
    current_buffer: Option<Buffer>,
    buffers: Vec<Buffer>,
}

impl BufferSystem {
    pub fn new() -> Box<Self> {
        Box::new(BufferSystem {
            current_buffer: None,
            buffers: Vec::new(),
        })
    }
}

impl ScalSystem for BufferSystem {
    fn run_system(&self, app_conf: &ApplicationConfig, win_handle: &mut ScalSDLWindow) {
    }
    fn update_system(&mut self,app_conf: &ApplicationConfig,  win_handle: &mut ScalSDLWindow) {
    }
    fn handle_input_system(&mut self,app_conf: &ApplicationConfig,  win_handle: &mut ScalSDLWindow, event: &Vec<Event>) {
    }
} 
