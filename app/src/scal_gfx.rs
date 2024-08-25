extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::TextureQuery;
use sdl2::rect::Rect;
use std::path::Path;
use std::time::Duration;

use crate::scal_ui;

// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

// Scale fonts to a reasonable size when they're too big (though they might look less smooth)
fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            println!("Scaling down! The text will look worse!");
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            println!("Scaling down! The text will look worse!");
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (rect_width as i32 - w) / 2;
    let cy = (rect_height as i32 - h) / 2;
    rect!(cx, cy, w, h)
}


#[allow(dead_code)]
pub struct ScalWindow {
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
    scal_ui: scal_ui::ScalUI,
}

impl ScalWindow {
    pub fn new(win_name: String, win_width: u32, win_height: u32) -> anyhow::Result<Self> {
        let sdl_context = sdl2::init()
            .map_err(|e| anyhow::anyhow!("sdl2 init error: {}", e))?;
        let video_subsystem = sdl_context.video()
            .map_err(|e| anyhow::anyhow!("sdl2 video subsystem error: {}", e))?;
        let event_pump = sdl_context.event_pump()
            .map_err(|e| anyhow::anyhow!("sdl2 event pump error: {}", e))?;
        let window = video_subsystem.window(&win_name, win_width, win_height)
            .resizable()
            .position_centered()
            .build()?;
        let ttf_context = sdl2::ttf::init().map_err(|e| anyhow::anyhow!("ttf context error: {}", e))?;

        let mut canvas = window.into_canvas().build()?;


        // test 
        let texture_creator = canvas.texture_creator();
        // Load a font
        let font_path = Path::new("../res/fonts/FiraCode-Regular.ttf");
        let mut font = ttf_context.load_font(font_path, 128)
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

        canvas.set_draw_color(Color::RGBA(195, 217, 255, 255));
        canvas.clear();

        let TextureQuery { width, height, .. } = texture.query();

        // If the example text is too big for the screen, downscale it (and center irregardless)
        let padding = 10;
        let target = get_centered_rect(
            width,
            height,
            width - padding,
            height - padding,
        );

        canvas.copy(&texture, None, Some(target))
            .map_err(|e| anyhow::anyhow!("error copying texture: {}", e))?;

        canvas.present();

        let scal_ui = scal_ui::ScalUI::new();

        Ok(ScalWindow{
            sdl_context,
            video_subsystem,
            canvas,
            event_pump,
            scal_ui,
        })
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        let mut i = 0;

        'running: loop {
            
            // Handle events
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running;
                    },
                    Event::KeyDown { keycode: Some(Keycode::Num8), .. } => {
                    },
                    _ => {}
                }
            }

            // Test: change background color
            //i = (i + 1) % 255;
            //self.canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
            //self.canvas.clear();
            //self.canvas.present();

            // ------------------------------
            // The rest of the graphics loop goes here...
            // ------------------------------
            
            // update session/buffer state 
            //self.scal_ui.render();
            

            // sleep for 1/60th of a second (60fps)
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
        Ok(())
    }
}
