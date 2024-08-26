mod scal_config;
mod scal_buffer;
mod scal_session;

mod scal_gfx;
mod scal_ui;

use env_logger::Env;

fn buffer_test() -> scal_buffer::Buffer {
    let buffer = scal_buffer::Buffer::new();

    let line1_text = "This is a scalic text editor";
    let line2_text = "this is a monkas test";
    let line3_text = "this is a test of the emergency broadcast system";

    let mut line1 = Vec::new();
    let mut line2 = Vec::new();
    let mut line3 = Vec::new();

    for c in line1_text.chars() {
        line1.push(scal_buffer::BufferCell{
            character: c,
            fg_color: (255, 255, 255, 255),
            bg_color: (0, 0, 0, 255),
            is_cursor_blink: false,
            is_wireframe: true,
        });
    }

    for c in line2_text.chars() {
        line2.push(scal_buffer::BufferCell{
            character: c,
            fg_color: (255, 255, 255, 255),
            bg_color: (0, 0, 0, 255),
            is_cursor_blink: false,
            is_wireframe: true,
        });
    }

    for c in line3_text.chars() {
        line3.push(scal_buffer::BufferCell{
            character: c,
            fg_color: (255, 255, 255, 255),
            bg_color: (0, 0, 0, 255),
            is_cursor_blink: false,
            is_wireframe: true,
        });
    }

    buffer
}

pub fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let _session = scal_session::ScalSession::new();
    let config = scal_config::ApplicationConfig::new_default();

    let mut win_handle = scal_gfx::ScalSDLWindow::new(config)?;

    // need an ui system here
    let mut scal_ui = scal_ui::ScalUI::new();

    let _test_view = Box::new(scal_ui::TestView);
    let buffer_view = Box::new(scal_ui::BufferView{buffer: buffer_test()});

    scal_ui.change_view(_test_view);

    // need an input system here
    // need an event system here

    // start the main rendering loop
    win_handle.start(&scal_ui)?;
    Ok(())
}
