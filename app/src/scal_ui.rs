use std::path::Path;

use sdl2::{pixels::Color, render::TextureQuery};
use sdl2::rect::Rect;

use crate::scal_buffer::Buffer;
use crate::scal_gfx::ScalSDLWindow;

pub trait ScalUIView {
    fn render(&self, win_handle: &mut ScalSDLWindow) -> anyhow::Result<()>;
} 
pub struct ScalUI {
    pub view: Option<Box<dyn ScalUIView>>,
}

impl ScalUI{
    pub fn new() -> Self {
        ScalUI { 
            view: None
        }
    }

    pub fn change_view(&mut self, new_view: Box<dyn ScalUIView>) {
        self.view = Some(new_view);
    }

    pub fn update(&self, win_handle: &mut ScalSDLWindow) {
        if let Some(view) = &self.view {
            let _ = view.render(win_handle);
        } 
    }
}


pub struct TestView;
pub struct BufferView{
    pub buffer: Buffer
}

impl ScalUIView for TestView {
    fn render(&self, win_handle: &mut ScalSDLWindow) -> anyhow::Result<()> {

        let canvas = &mut win_handle.canvas;
        let ttf_context = &mut win_handle.ttf_context;
        let texture_creator = &mut win_handle.texture_creator;

        let font_path = Path::new("../res/fonts/FiraCode-Regular.ttf");
        let mut font = ttf_context.load_font(font_path, 16)
            .map_err(|e| anyhow::anyhow!("error loading font: {}", e))?;
        font.set_style(sdl2::ttf::FontStyle::NORMAL);

        // render a surface, and convert it to a texture bound to the canvas
        let surface = font
            .render("Scalic Text Editor")
            .blended(Color::RGBA(255, 0, 0, 255))
            .map_err(|e| anyhow::anyhow!("error rendering ttf surface: {}", e))?;

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| anyhow::anyhow!("error rendering ttf texture: {}", e))?;

        canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        canvas.clear();

        let TextureQuery { width, height, .. } = texture.query();
        let tex_width = width;
        let tex_height = height;

        let screen_width = canvas.output_size().unwrap().0;
        let screen_height = canvas.output_size().unwrap().1;

        let padding = 0;
        let target = Rect::new(
            padding,
            padding,
            tex_width,
            tex_height,
        );

        canvas.copy(&texture, None, Some(target))
            .map_err(|e| anyhow::anyhow!("error copying texture: {}", e))?;

        Ok(())
    }
}

impl ScalUIView for BufferView {
    fn render(&self, win_handle: &mut ScalSDLWindow) -> anyhow::Result<()>{

        let canvas = &mut win_handle.canvas;
        let ttf_context = &mut win_handle.ttf_context;
        let texture_creator = &mut win_handle.texture_creator;

        let font_path = Path::new("../res/fonts/FiraCode-Regular.ttf");
        let mut font = ttf_context.load_font(font_path, 16)
            .map_err(|e| anyhow::anyhow!("error loading font: {}", e))?;
        font.set_style(sdl2::ttf::FontStyle::NORMAL);



        // render a surface, and convert it to a texture bound to the canvas
        let surface = font
            .render("Scalic Text Editor")
            .blended(Color::RGBA(255, 0, 0, 255))
            .map_err(|e| anyhow::anyhow!("error rendering ttf surface: {}", e))?;

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| anyhow::anyhow!("error rendering ttf texture: {}", e))?;

        canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        canvas.clear();

        let TextureQuery { width, height, .. } = texture.query();
        let tex_width = width;
        let tex_height = height;

        let screen_width = canvas.output_size().unwrap().0;
        let screen_height = canvas.output_size().unwrap().1;

        let padding = 0;
        let target = Rect::new(
            padding,
            padding,
            tex_width,
            tex_height,
        );

        canvas.copy(&texture, None, Some(target))
            .map_err(|e| anyhow::anyhow!("error copying texture: {}", e))?;

        Ok(())
    }
}
