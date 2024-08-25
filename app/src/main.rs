
mod scal_buffer;
mod scal_session;

mod scal_gfx;
mod scal_ui;

use env_logger::Env;

pub fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let _session = scal_session::ScalSession::new();

    let win_name: String = String::from("Scalic - Text Editor");
    let win_width: u32 = 800;
    let win_height: u32 = 600;
    let mut win_handle = scal_gfx::ScalWindow::new(win_name, win_width, win_height)?;

    // get reference to window_handle across all libraries that need it (e.g. scal_ui)

    // start the main rendering loop
    win_handle.start()?;
    Ok(())
}
