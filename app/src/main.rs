mod scal_config;
mod scal_buffer;
mod scal_session;

mod scal_gfx;
mod scal_ui;

use env_logger::Env;
use scal_buffer::Buffer;


pub fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let mut config = scal_config::ApplicationConfig::load_config();

    if config.is_err() {
        let err = config.err().unwrap();    
        log::error!("Failed to load config file: {:?}", err);
        config = Ok(scal_config::ApplicationConfig::new_default());
    }

    let mut win_handle = scal_gfx::ScalSDLWindow::new(config?)?;

    // need an ui system here
    let mut scal_ui = scal_ui::ScalUI::new();

    let _test_view = Box::new(scal_ui::TestView);
    let _buffer_view_1 = Box::new(scal_ui::BufferView{buffer: Buffer::new_test_one()});
    let buffer_view_2 = Box::new(scal_ui::BufferView{buffer: Buffer::read("test_file.txt")?});

    scal_ui.change_view(buffer_view_2);

    // need an input system here
    // need an event system here

    // start the main rendering loop
    win_handle.start(&scal_ui)?;
    Ok(())
}
