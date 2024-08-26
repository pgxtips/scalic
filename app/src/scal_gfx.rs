extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use crate::scal_config::ApplicationConfig;
use crate::scal_ui::ScalUI;

#[allow(dead_code)]
pub struct ScalSDLWindow {
    pub app_conf: ApplicationConfig,
    pub sdl_context: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub event_pump: sdl2::EventPump,
    pub texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    pub ttf_context: sdl2::ttf::Sdl2TtfContext,
}

impl ScalSDLWindow {
    pub fn new(app_conf: ApplicationConfig) -> anyhow::Result<Self> {
        let sdl_context = sdl2::init()
            .map_err(|e| anyhow::anyhow!("sdl2 init error: {}", e))?;
        let video_subsystem = sdl_context.video()
            .map_err(|e| anyhow::anyhow!("sdl2 video subsystem error: {}", e))?;
        let event_pump = sdl_context.event_pump()
            .map_err(|e| anyhow::anyhow!("sdl2 event pump error: {}", e))?;
        let window = video_subsystem.window(&app_conf.window_name, app_conf.window_width, app_conf.window_height)
            .resizable()
            .position_centered()
            .build()?;
        let ttf_context = sdl2::ttf::init().map_err(|e| anyhow::anyhow!("ttf context error: {}", e))?;
        let canvas = window.into_canvas().build()?;
        let texture_creator = canvas.texture_creator();

        Ok(ScalSDLWindow{
            app_conf,
            sdl_context,
            video_subsystem,
            canvas,
            event_pump,
            texture_creator,
            ttf_context,
        })
    }

    pub fn start(&mut self, scal_ui: &ScalUI) -> anyhow::Result<()> {
        'running: loop {
            
            // Handle events
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running;
                    },
                    _ => {}
                }
            }

            self.canvas.clear();
            scal_ui.update(self);
            self.canvas.present();

            // sleep for 1/60th of a second (60fps)
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
        Ok(())
    }
}
