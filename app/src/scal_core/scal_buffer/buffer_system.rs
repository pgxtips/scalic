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
    fn run(&self, app_conf: &ApplicationConfig, win_handle: &mut ScalSDLWindow) {
    }
    fn update(&self, win_handle: &mut ScalSDLWindow) {
    }
    fn handle_input(&self, win_handle: &mut ScalSDLWindow) {
    }
} 
