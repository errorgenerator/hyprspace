use std::fs;
use directories::BaseDirs;

// standard name for the $PATH variable
static OS_PATH_VAR_NAME: &str = "PATH";

// standard directory for .desktop files
static APPLICATION_DESKTOP_FILES_DIRECTORY: &str = "/usr/share/applications/";

static APPLICATION_ICONS_SYSTEM_WIDE_DIRECTORY: &str = "/usr/share/icons/";

static APPLICATION_PIXMAPS_DIRECTORY: &str = "/usr/share/pixmaps/";

// hyprspace main window label 
static HYPRSPACE_LABEL: &str = "hyprspace";

// prefferred terminal emulator
static HYPRSPACE_PREF_TERM_EMULATOR: &str = "alacritty";

// standard ok response over socket
static HYPRSPACE_OK_SOCK_RESPONSE: &str = "OK";

// standard err response over socket
static HYPRSPACE_ERR_SOCK_RESPONSE: &str = "ERR";

// command for showing the window, when communicating over the socket
static HYPRSPACE_SHOW_SOCK_COMMAND: &str = "SHOW";

// command for hiding the window, when communicating over the socket
static HYPRSPACE_HIDE_SOCK_COMMAND: &str = "HIDE";

// command to toggle the window state, when communicating over the socket
static HYPRSPACE_TOGGLE_SOCK_COMMAND: &str = "TOGGLE";

// socket file location
static HYPRSPACE_SOCK_FILE_PATH: &str = "/tmp/hyprspace.sock";

// directory name for config files in .config
static HYPRSPACE_CONFIG_DIR_NAME: &str = "hyprspace";

// standard hyrpspace config file name
static HYPRSPACE_CONFIG_FILE_NAME: &str = "hyprspace.toml";

// standard hyprspace css style sheet name
static HYPRSPACE_STYLE_FILE_NAME: &str = "hypr.css";

// standard name for application template file 
static HYPRSPACE_APPLICATION_TEMPLATE_FILE_NAME: &str = "hypr_fav_app.html";

// the terminal to use for shell executables
static HYPRSPACE_PREFFERRED_TERMINAL_EMULATOR: &str = "alacritty";

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AppConfiguration {

}

pub fn load_app_configuration() -> AppConfiguration {
    // get the config dir first
    let base_dirs = match BaseDirs::new() {
        None => panic!("Unable to determine Base-Directories, something feels off"),
        Some(dirs) => dirs
    };

    let config_dir = base_dirs.config_dir();

    let file_path = format!("{}/{}/{}", config_dir.display(), HYPRSPACE_CONFIG_DIR_NAME, HYPRSPACE_CONFIG_FILE_NAME);
    if config_file_exists(file_path.clone().as_str()) {
        return load_config_from_file();
    }
    create_config_from_defaults()
}

fn create_config_from_defaults() -> AppConfiguration {
    AppConfiguration {  }
}

fn load_config_from_file() -> AppConfiguration {
    AppConfiguration {  }
}

fn config_file_exists(path_to_config: &str) -> bool {
    fs::metadata(path_to_config).is_ok()
}