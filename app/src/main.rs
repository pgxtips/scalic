use env_logger::Env;
use scal_core::{scal_application::{builder::ApplicationBuilder, configuration::ApplicationConfig}, scal_buffer::buffer_system::BufferSystem, scal_event::event_manager::EventManager};
use scal_core::scal_ui::ui_system::UISystem;

mod scal_core;

pub fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let config = match ApplicationConfig::load_config(){
        Ok(conf) => conf,
        Err(err) => {
            log::error!("Failed to load config file: {:?}", err);
            ApplicationConfig::new_default()
        }
    };

    let event_manager = EventManager::new();
    let buffer_system = BufferSystem::new();
    let ui_system = UISystem::new();

    let mut application = ApplicationBuilder::new()
        .set_config(config)
        .set_event_manager(event_manager)
        .add_system(buffer_system)
        .add_system(ui_system)
        .build()?;

    let _ = application.run();

    Ok(())
}
