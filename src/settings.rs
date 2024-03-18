/// settings.rs
///
/// 2024
/// author Marcel Dudek

use std::default::Default;
use std::{env, error, fs};
use yaml_rust2::YamlLoader;

/// Command that will be used if no YAML configuration is loaded.
const DEFAULT_COMMAND: &'static str = "swww img ${FILE_PATH}";

/// Wallpapers folder that will be used if no YAML configuration is loaded.
const DEFAULT_WALLPAPERS_FOLDER_PATH: &'static str = "Pictures/";

/// Settings for walpsel application.
pub struct Settings {
    /// Command to be executed when changing wallpaper.
    /// File path will be inserted under ${FILE_PATH}.
    pub command: String,

    /// Path to folder containing wallpapers.
    pub wallpapers_folder_path: String,
}

impl Settings {
    /// Load configuration from YAML file stored under path.
    ///
    /// Returns Result containing Settings struct if successful.
    pub fn load(path: &str) -> Result<Self, Box<dyn error::Error>> {
        let settings_contents = fs::read_to_string(path)?;
        let yaml = YamlLoader::load_from_str(&settings_contents)?;
        let yaml = &yaml[0];
        let default_wallpapers_folder = Self::get_default_wallpapers_folder_full_path();
        Ok(Self {
            command: yaml["command"]
                .as_str()
                .unwrap_or(DEFAULT_COMMAND)
                .to_string(),
            wallpapers_folder_path: yaml["wallpapers_folder_path"]
                .as_str()
                .unwrap_or(&default_wallpapers_folder)
                .to_string(),
        })
    }

    /// Get default path to wallpapers folder.
    fn get_default_wallpapers_folder_full_path() -> String {
        format!(
            "{}/{}",
            env::var_os("HOME").unwrap().into_string().unwrap(),
            DEFAULT_WALLPAPERS_FOLDER_PATH
        )
    }
}


impl Default for Settings {
    /// Get default Settings struct.
    fn default() -> Self {
        Self {
            command: DEFAULT_COMMAND.to_string(),
            wallpapers_folder_path: Self::get_default_wallpapers_folder_full_path(),
        }
    }
}

