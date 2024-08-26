
use std::path::Path;

use sdl2::{pixels::Color, rect::Rect, render::TextureQuery};

use crate::scal_core::{scal_application::configuration::ApplicationConfig, scal_buffer::buffer::Buffer, scal_ui::ui_component::UIComponent, scal_window::ScalSDLWindow};

pub struct ComponentTextBuffer{
    pub buffer: Option<Buffer>, 
}

impl UIComponent for ComponentTextBuffer {
    fn draw(&self, app_conf: &ApplicationConfig, win_handle: &mut ScalSDLWindow) -> anyhow::Result<()> {

        if self.buffer.is_none() { return Ok(()); }
        let buffer = self.buffer.as_ref().unwrap();
        
        let canvas = &mut win_handle.canvas;
        let ttf_context = &mut win_handle.ttf_context;
        let texture_creator = &mut win_handle.texture_creator;

        // we want a high point size to get a good resolution, however,
        // we want to render the font to a given font size
        let font_size = app_conf.font_size as f32;

        // does nothing atm, but in future I might need to manually set the 
        // font point size to get a better resolution as scale according to 
        // the font size
        let font_point_size = (font_size as f32 * 1.0) as u16;

        let font_path = Path::new(&app_conf.font_path);
        let mut font = ttf_context.load_font(font_path, font_point_size)
            .map_err(|e| anyhow::anyhow!("error loading font: {}", e))?;
        font.set_style(sdl2::ttf::FontStyle::NORMAL);


        // ** note **
        // font_point_size is the size of the font we are rendering, but 
        // we want to render the font to a given font_size
        // so we need to scale the texture to match the font_size
        let tex_scale_factor = font_size as f32 / font_point_size as f32; 

        let buffer_cells = &buffer.buffer_cells;

        for (row_idx, row) in buffer_cells.iter().enumerate() {
            for (col_idx, _) in row.iter().enumerate() {

                let cell = &buffer_cells[row_idx][col_idx]; 
                let character = cell.character;
                let fg_color = cell.fg_color;
                let bg_color = cell.bg_color;

                // render a surface, and convert it to a texture bound to the canvas
                let surface = font
                    .render(&character.to_string())
                    .shaded(
                        Color::RGBA(fg_color.0, fg_color.1, fg_color.2, fg_color.3),
                        Color::RGBA(bg_color.0, bg_color.1, bg_color.2, bg_color.3)
                    )
                    .map_err(|e| anyhow::anyhow!("error rendering ttf surface: {}", e))?;

                let tex = texture_creator
                    .create_texture_from_surface(&surface)
                    .map_err(|e| anyhow::anyhow!("error rendering ttf texture: {}", e))?;

                let TextureQuery { width, height, .. } = tex.query();

                // scale down the texture to match font size
                let tex_width = width;
                let tex_height = height;

                let scaled_width = (tex_width as f32 * tex_scale_factor) as i32;
                let scaled_height = (tex_height as f32 * tex_scale_factor) as i32;

                // cell x and y position
                let cx = col_idx as i32 * scaled_width;
                let cy = row_idx as i32 * scaled_height;

                let target = Rect::new(
                    cx,
                    cy,
                    scaled_width as u32,
                    scaled_height as u32,
                );

                // Apply blend mode for smoother text rendering
                canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

                canvas.copy(&tex, None, Some(target))
                    .map_err(|e| anyhow::anyhow!("error copying texture: {}", e))?;
            }
        }
        Ok(())
    }
    fn update(&self) {
        "TextBuffer".to_string();
    }
    fn handle_input(&self) {
        "TextBuffer".to_string();
    }
}
