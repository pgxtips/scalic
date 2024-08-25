extern crate scal_gfx;

use env_logger::{self, Env};


pub fn application_init(){
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    log::info!("application init");

    let win_name: String = String::from("Scalic - Text Editor");
    let win_width: u32 = 800;
    let win_height: u32 = 600;
    scal_gfx::win_init(win_name, win_width, win_height);
}

pub fn main() {
    application_init();
}
