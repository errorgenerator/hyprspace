extern crate directories;
use std::fs;

use directories::BaseDirs;

use super::{create_app_configuration, defaults::HYPRSPACE_CONFIG_DIR_NAME};

#[tauri::command]
pub fn get_style_sheet_path() -> String {
    let cached_config = create_app_configuration(None);

    // let's get the directories
    let base_dirs = match BaseDirs::new() {
        None => panic!("Unable to determine Base-Directories, something feels off..."),
        Some(dirs) => dirs,
    };

    let config_dir = base_dirs.config_dir();

    let file_path = format!(
        "{}/{}/{}",
        config_dir.display(),
        HYPRSPACE_CONFIG_DIR_NAME,
        cached_config.config.style_sheet.unwrap()
    );

    if fs::metadata(file_path.clone()).is_ok() {
        return file_path;
    }

    "".to_string()
}
