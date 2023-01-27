pub mod defaults;
pub mod load_style;
pub mod parse_path;

use cached::proc_macro::cached;
use directories::BaseDirs;
use std::fs;

use defaults::{
    HYPRSPACE_CONFIG_DIR_NAME, 
    HYPRSPACE_CONFIG_FILE_NAME,
    APPLICATION_DESKTOP_FILES_DIRECTORY, 
    OS_PATH_VAR_NAME, 
    HYPRSPACE_SOCK_FILE_PATH, 
    HYPRSPACE_STYLE_FILE_NAME,
    HYPRSPACE_APPLICATION_TEMPLATE_FILE_NAME,
    HYPRSPACE_PREF_TERM_EMULATOR
};



/// This Struct holds the configuration for the entire application
/// It is parsed from a file called hyprspace.toml.
/// Hyprspace will look for hyprspace.toml in the following directory: /home/username/.config/hyprspace/
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AppConfiguration {
    pub applications: Applications,
    pub socket: Socket,
    pub config: Config,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Applications {
    pub path_var: Option<String>,
    pub desktop_files_dir: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Socket {
    pub socket_file_path: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub style_sheet: Option<String>,
    pub application_template: Option<String>,
    pub preferred_terminal_emulator: Option<String>,
}

#[cached(size=1)]
pub fn create_app_configuration(optional_config_dir_path: Option<String>) -> AppConfiguration {
    let configuration = match load_config_if_exists(optional_config_dir_path) {
        None => create_app_config_from_defaults(),
        Some(c) => c
    };

    configuration
}

fn create_app_config_from_defaults() -> AppConfiguration {
    let applications = Applications {
        path_var: Option::Some(String::from(OS_PATH_VAR_NAME)),
        desktop_files_dir: Option::Some(String::from(APPLICATION_DESKTOP_FILES_DIRECTORY)),
    };
    let socket = Socket {
        socket_file_path: Option::Some(String::from(HYPRSPACE_SOCK_FILE_PATH)),
    };
    let config = Config {
        style_sheet: Option::Some(String::from(HYPRSPACE_STYLE_FILE_NAME)),
        application_template: Option::Some(String::from(HYPRSPACE_APPLICATION_TEMPLATE_FILE_NAME)),
        preferred_terminal_emulator: Option::Some(String::from(HYPRSPACE_PREF_TERM_EMULATOR))
    };

    AppConfiguration { applications: applications, socket: socket, config: config }
}

fn load_config_if_exists(optional_config_dir_path: Option<String>) -> Option<AppConfiguration> {
    let hyprspace_config_dir_name = String::from(HYPRSPACE_CONFIG_DIR_NAME);
    let hyprspace_config_file_name = String::from(HYPRSPACE_CONFIG_FILE_NAME);

    let base_dirs = match BaseDirs::new() {
        None => {
            error!("Unable to load base directories!");
            return None;
        }
        Some(dirs) => dirs,
    };

    let config_dir = base_dirs.config_dir();

    let file_path = match optional_config_dir_path {
        None => {
            format!(
                "{}/{}/{}",
                config_dir.display(),
                hyprspace_config_dir_name,
                hyprspace_config_file_name
            )
        },
        Some(s) => {
            format!(
                "{}/{}",
                s,
                hyprspace_config_file_name
            )
        }
    };

    if config_file_exists(file_path.clone().as_str()) {
        let config_file_contents = match fs::read_to_string(file_path.clone()) {
            Err(why) => {
                error!("Unable to read config file {}!\nError: {}", file_path, why);
                return None
            },
            Ok(s) => s
        };

        let config: AppConfiguration = match toml::from_str(config_file_contents.as_str()) {
            Err(why) => {
                println!("Unable to parse Config-File!\nError {}", why.clone());
                error!("Unable to parse Config-File!\nError {}", why);
                return None;
            },
            Ok(c) => c
        };

        let filled_config = fill_configuration(config);

        return Some(filled_config.clone());
    }

    None
}

fn fill_configuration(app_config: AppConfiguration) -> AppConfiguration {

    let mut applications = app_config.applications.clone();
    let mut socket = app_config.socket.clone();
    let mut config = app_config.config.clone();


    match applications.path_var {
        None => {
            applications.path_var = Option::Some(String::from(OS_PATH_VAR_NAME));
        },
        Some(p) => {
            applications.path_var = Option::Some(p.clone());
        }
    }

    match applications.desktop_files_dir {
        None => {
            applications.desktop_files_dir = Option::Some(String::from(APPLICATION_DESKTOP_FILES_DIRECTORY));
        },
        Some(p) => {
            applications.desktop_files_dir = Option::Some(p.clone());
        }
    }

    match socket.socket_file_path {
        None => {
            socket.socket_file_path = Option::Some(String::from(HYPRSPACE_SOCK_FILE_PATH));
        },
        Some(p) => {
            socket.socket_file_path = Option::Some(p.clone());
        }
    }

    match config.application_template {
        None => {
            config.application_template = Option::Some(String::from(HYPRSPACE_APPLICATION_TEMPLATE_FILE_NAME));
        },
        Some(p) => {
            config.application_template = Option::Some(p.clone());
        }
    }

    match config.style_sheet {
        None => {
            config.style_sheet = Option::Some(String::from(HYPRSPACE_STYLE_FILE_NAME));
        },
        Some(p) => {
            config.style_sheet = Option::Some(p.clone());
        }
    }

    match config.preferred_terminal_emulator {
        None => {
            config.preferred_terminal_emulator = Option::Some(String::from(HYPRSPACE_PREF_TERM_EMULATOR));
        },
        Some(p) => {
            config.preferred_terminal_emulator = Option::Some(p.clone());
        }
    }

    AppConfiguration { applications: applications.clone(), socket: socket.clone(), config: config.clone() }
}

fn config_file_exists(path_to_config: &str) -> bool {
    fs::metadata(path_to_config).is_ok()
}
