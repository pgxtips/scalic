extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

#[allow(dead_code)]
pub struct ScalWindow {
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
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

        let canvas = window.into_canvas().build()?;

        Ok(ScalWindow{
            sdl_context,
            video_subsystem,
            canvas,
            event_pump,
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
                    _ => {}
                }
            }

            // Test: change background color
            i = (i + 1) % 255;
            self.canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
            self.canvas.clear();
            self.canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

            // The rest of the graphics loop goes here...
        }
        Ok(())
    }
}
