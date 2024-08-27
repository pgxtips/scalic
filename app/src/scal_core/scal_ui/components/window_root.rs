use sdl2::event::Event;

use crate::scal_core::{scal_application::configuration::ApplicationConfig, scal_ui::ui_component::UIComponent, scal_window::ScalSDLWindow};

pub struct ComponentWindowRoot{
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub colour: (u8, u8, u8, u8),
}

impl ComponentWindowRoot {
    pub fn new(x: i32, y: i32, width: i32, height: i32, colour: (u8, u8, u8, u8)) -> ComponentWindowRoot {
        ComponentWindowRoot {
            x,
            y,
            width,
            height,
            colour
        }
    }
}

impl UIComponent for ComponentWindowRoot {
    fn draw(&self, app_conf: &ApplicationConfig, win_handle: &mut ScalSDLWindow) -> anyhow::Result<()>{
        let canvas = &mut win_handle.canvas;
        canvas.set_draw_color(self.colour);
        Ok(())
    }
    fn update(&mut self, app_conf: &ApplicationConfig, _win_handle: &mut ScalSDLWindow) -> anyhow::Result<()> { Ok(()) }
    fn handle_input(&mut self,  app_conf: &ApplicationConfig, _win_handle: &mut ScalSDLWindow, events: &Vec<Event>) -> anyhow::Result<()>{ Ok(()) }
}
