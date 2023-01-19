use crate::applications::application::Application;

pub mod load_style;
pub mod parse_path;

/// This Struct holds the configuration for the entire application
/// It is parsed from a file called hyprspace.toml.
/// Hyprspace will look for hyprspace.toml in the following directories: /home/username/.config/hyprspace
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HyprspaceConfig {
    pub socket_file_path: String,
    pub style_sheet_file_name: String,
    pub application_html_template_name: String,
    pub fav_apps: Vec<Application>,
}