/// walpsel_app.rs
///
/// 2024
/// author Marcel Dudek

use crate::wallpaper_engine::WallpaperEngine;
use eframe::egui;
use egui_extras;
use std::fs;

/// Walpsel app implementation for eframe.
pub struct WalpselApp {
    /// Path to folder containing wallpaper images.
    path: String,

    /// Engine used to change wallpaper.
    engine: WallpaperEngine,
}

impl WalpselApp {
    /// Create new instance of WalpselApp that will use
    /// provided wallpaper_eng and search for images under
    /// path_.
    pub fn new(path_: &str, wallpaper_eng: WallpaperEngine) -> Self {
        Self {
            path: path_.to_string(),
            engine: wallpaper_eng,
        }
    }

    /// Get images under the path.
    ///
    /// Returns vector of paths to images.
    fn get_images(&self) -> Vec<String> {
        let mut files = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_path = entry.path();
                    if WalpselApp::is_image_type(&file_path) {
                        if let Some(path_string) = file_path.to_str() {
                            files.push(String::from(path_string));
                        }
                    }
                }
            }
        }
        files
    }

    /// Check if file under file_path is an image type.
    /// Image types supported: jpg, jpeg, png, bmp, gif.
    ///
    /// Returns boolean value.
    fn is_image_type(file_path: &std::path::PathBuf) -> bool {
        if let Some(extension) = file_path.extension() {
            if let Some(ext_str) = extension.to_str() {
                const IMAGE_EXTENSIONS: &'static [&'static str] =
                    &["jpg", "jpeg", "png", "bmp", "gif"];
                return IMAGE_EXTENSIONS
                    .iter()
                    .any(|&ext| ext == ext_str.to_lowercase());
            }
        }
        false
    }
}

impl eframe::App for WalpselApp {
    /// Implementation of __eframe::App::update__.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::default().inner_margin(20.0).outer_margin(20.0))
            .show(ctx, |ui| {
                ui.with_layout(
                    egui::Layout::centered_and_justified(egui::Direction::TopDown),
                    |ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            // show images
                            egui_extras::install_image_loaders(ctx);
                            const MIN_IMAGE_WIDTH: f32 = 300.0;
                            let rows = (ui.available_width() / MIN_IMAGE_WIDTH) as i32;
                            egui::Grid::new("images_grid")
                                .max_col_width(ui.available_width() / rows as f32)
                                .min_row_height(300.0)
                                .show(ui, |ui| {
                                    let mut ctr = 0;
                                    for image in self.get_images() {
                                        let im = egui::Image::new("file://".to_owned() + &image);
                                        let response = ui.add(egui::widgets::ImageButton::new(im));
                                        if response.clicked() {
                                            match self.engine.change_wallpaper(&image) {
                                                Ok(_) => (),
                                                Err(e) => {
                                                    println!("Error while chaning wallpaper: {}", e)
                                                }
                                            }
                                        }
                                        ctr += 1;
                                        if ctr == rows {
                                            ui.end_row();
                                            ctr = 0;
                                        }
                                    }
                                });
                        });
                    },
                );
            });
    }
}
