pub struct ApplicationConfig {
    pub window_name : String,
    pub window_width: u32,
    pub window_height: u32,
    pub font_size: u32,
    pub font_name: String,
    pub font_path: String,
}

impl ApplicationConfig {
    /// Create a new ApplicationParams with default values
    pub fn new_default() -> Self {
        ApplicationConfig {
            window_name: "Scalic - Text Editor".to_string(),
            window_width: 800,
            window_height: 600,
            font_size: 12,
            font_name: "FiraCode".to_string(),
            font_path: "../res/fonts/FiraCode-Regular.ttf".to_string(),
        }
    }

    /// Load the application configuration from a file
    pub fn load_config() -> Self {
        ApplicationConfig {
            window_name: "Scalic - Text Editor".to_string(),
            window_width: 800,
            window_height: 600,
            font_size: 12,
            font_name: "FiraCode".to_string(),
            font_path: "../res/fonts/FiraCode-Regular.ttf".to_string(),
        }
    }
}
