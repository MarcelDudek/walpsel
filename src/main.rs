/// main.rs
///
/// 2024
/// author Marcel Dudek

use settings::Settings;
use std::env;
use wallpaper_engine::WallpaperEngine;
use walpsel_app::WalpselApp;

mod settings;
mod wallpaper_engine;
mod walpsel_app;

/// Path to configuration file.
const CONFIG_PATH: &'static str = ".config/walpsel/settings.yaml";

/// Entry point of the program.
fn main() {
    // load settings
    let config_path = format!(
        "{}/{}",
        env::var_os("HOME").unwrap().into_string().unwrap(),
        CONFIG_PATH
    );
    let app_settings = Settings::load(&config_path).unwrap_or_else(|_| Settings::default());

    // create application object
    let app = WalpselApp::new(
        &app_settings.wallpapers_folder_path,
        WallpaperEngine::new(&app_settings.command),
    );

    // run eframe
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            // .with_inner_size([320.0, 240.0])
            .with_min_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "walpsel",
        options,
        Box::new(|_cc| Box::<WalpselApp>::new(app)),
    );
}
