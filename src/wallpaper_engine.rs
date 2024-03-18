/// wallpaper_engine.rs
///
/// 2024
/// author Marcel Dudek

use std::error;
use std::process::Command;

/// String which is replaced by actual file path in command string.
pub const FILE_PATH_REPLACEMENT_STR: &'static str = "${FILE_PATH}";

/// Wallpaper engine changes the Wallpaper using provided command.
pub struct WallpaperEngine {
    /// Command splitted into arguments.
    args: Vec<String>,
}

impl WallpaperEngine {
    /// Create new wallpaper engine that will use provided
    /// command to change wallpapers.
    ///
    /// To indicate place where wallpaper path should be inserted
    /// use FILE_PATH_REPLACEMENT_STR in the command string.
    pub fn new(command: &str) -> Self {
        Self {
            args: command.split_whitespace().map(|s| s.to_string()).collect(),
        }
    }

    /// Change wallpaper to the one provided in image_path.
    ///
    /// Returns Result that will indicate error if something
    /// goes wrong.
    pub fn change_wallpaper(&self, image_path: &str) -> Result<(), Box<dyn error::Error>> {
        // compile arguments for command
        let mut args_temp: Vec<String> = Vec::with_capacity(self.args.len());
        for arg in &self.args {
            let a = if let Some(index) = arg.find(FILE_PATH_REPLACEMENT_STR) {
                let mut arg_replace = arg.clone();
                arg_replace
                    .replace_range(index..index + FILE_PATH_REPLACEMENT_STR.len(), image_path);
                arg_replace
            } else {
                arg.clone()
            };
            args_temp.push(a);
        }

        // run command
        let mut cmd = Command::new(&args_temp[0])
            .args(&args_temp[1..])
            .spawn()
            .expect(format!("Failed to spawn command: {}", args_temp.join(" ")).as_str());

        let cmd_exit_status = cmd
            .wait()
            .expect(format!("Couldn't wait for command: {}", args_temp.join(" ")).as_str());

        if cmd_exit_status.success() {
            Ok(())
        } else {
            Err(format!(
                "Couldn't wait for command: {}",
                args_temp.join(" ")
            ))?
        }
    }
}
