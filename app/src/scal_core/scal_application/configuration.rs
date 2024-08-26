use std::{fs::File, io::{BufReader, Read}, path::Path};

#[derive(Debug)]
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
    pub fn load_config() -> anyhow::Result<Self> {

        let conf_path = Path::new("scalic.conf");
        let conf_file = File::open(conf_path)?;
        let mut reader = BufReader::new(conf_file);
        let mut file_data = String::new();
        reader.read_to_string(&mut file_data)?;
        let lines: Vec<String> = file_data.lines().map(|s| s.to_string()).collect();


        let mut window_width = 800;
        let mut window_height = 600;
        let mut font_size = 12;
        let mut font_name = "FiraCode".to_string();
        let mut font_path = "fonts/FiraCode-Regular.ttf".to_string();

        for line in lines {
            let parts: Vec<&str> = line.split("=").collect();
            if parts.len() != 2 {
                return Err(anyhow::Error::msg("Invalid line in config file: {line}"));
            }
            let key = parts[0].trim();
            let value = parts[1].trim();

            match key {
                "window_width" => {
                    let width = value.parse::<u32>()?;
                    if width < 800 {
                        return Err(anyhow::Error::msg("Window width must be at least 800"));
                    }
                    window_width = width;
                },

                "window_height" => {
                    let height = value.parse::<u32>()?;
                    if height < 600 {
                        return Err(anyhow::Error::msg("Window height must be at least 600"));
                    }
                    window_height = height;
                },

                "font_size" => {
                    let size = value.parse::<u32>()?;
                    if size < 8 {
                        return Err(anyhow::Error::msg("Font size must be at least 8"));
                    }
                    font_size = size;
                },

                "font_name" => { font_name = value.to_string(); },
                "font_path" => { font_path = value.to_string(); },
                _ => { return Err(anyhow::Error::msg("Invalid key in config file")); }
            }
        }

        Ok(ApplicationConfig {
            window_name: "Scalic - Text Editor".to_string(),
            window_width,
            window_height,
            font_size,
            font_name,
            font_path,
        })
    }
}
