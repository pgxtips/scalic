extern crate sdl2;

use crate::scal_core::scal_application::configuration::ApplicationConfig;

#[allow(dead_code)]
pub struct ScalSDLWindow {
    pub sdl_context: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub event_pump: sdl2::EventPump,
    pub texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    pub ttf_context: sdl2::ttf::Sdl2TtfContext,
}

impl ScalSDLWindow {
    pub fn new(app_conf: &ApplicationConfig) -> anyhow::Result<Self> {
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
            sdl_context,
            video_subsystem,
            canvas,
            event_pump,
            texture_creator,
            ttf_context,
        })
    }
}
