// standard name for the $PATH variable
pub static OS_PATH_VAR_NAME: &str = "PATH";

// standard directory for .desktop files
pub static APPLICATION_DESKTOP_FILES_DIRECTORY: &str = "/usr/share/applications/";

// hyprspace main window label 
pub static HYPRSPACE_LABEL: &str = "hyprspace";

// prefferred terminal emulator
pub static HYPRSPACE_PREF_TERM_EMULATOR: &str = "alacritty";

// standard ok response over socket
pub static HYPRSPACE_OK_SOCK_RESPONSE: &str = "OK";

// standard err response over socket
pub static HYPRSPACE_ERR_SOCK_RESPONSE: &str = "ERR";

// command for showing the window, when communicating over the socket
// pub static HYPRSPACE_SHOW_SOCK_COMMAND: &str = "SHOW";

// command for hiding the window, when communicating over the socket
// pub static HYPRSPACE_HIDE_SOCK_COMMAND: &str = "HIDE";

// command to toggle the window state, when communicating over the socket
// pub static HYPRSPACE_TOGGLE_SOCK_COMMAND: &str = "TOGGLE";

// socket file location
pub static HYPRSPACE_SOCK_FILE_PATH: &str = "/tmp/hyprspace.sock";

// directory name for config files in .config
pub static HYPRSPACE_CONFIG_DIR_NAME: &str = "hyprspace";

// standard hyrpspace config file name
pub static HYPRSPACE_CONFIG_FILE_NAME: &str = "hyprspace.toml";

// standard hyprspace css style sheet name
pub static HYPRSPACE_STYLE_FILE_NAME: &str = "hypr.css";

// standard name for application template file 
pub static HYPRSPACE_APPLICATION_TEMPLATE_FILE_NAME: &str = "hypr_fav_app.html";

// standard amount of applications to show in search results
pub static HYPRSPACE_APPLICATION_RESULTS_LIMIT: u32 = 5;