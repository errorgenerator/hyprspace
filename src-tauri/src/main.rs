// TODO: Clean up Code
// TODO: Change this config
// TODO: gotta use a Boyer-Moore algorithm for searching applications.
#![cfg_attr(
    all(not(debug_assertions), target_os = "linux"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate log;

mod command;
mod events;
mod socket;
mod config;
mod applications;

use std::process;

use events::{
    react_to_error_message, 
    search::get_search_results
};
use socket::start_listening_thread_on_socket;
use tauri::Manager;

use crate::{
    command::HyprSpaceCommand, 
    config::load_style::get_style_sheet_path, applications::load_apps_list, events::execute::handle_execution_request
};


// ### GLOBALS ###

//TODO: These need to eventually migrate to the Config file

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
static HYPRSPACE_PREFFERRED_TERMINAL_EMULATOR: &str = "alacritty"; //TODO: change to xterm


// ### Setup ###
fn main() {

    // start up logging framework
    env_logger::init();
    info!("---------- BEGIN LOG ----------");

    // initialize the application cache
    load_apps_list(false);

    let available_commands = vec![
        HyprSpaceCommand {
            cmd_name: "help".to_string(),
            cmd_function: command::help_cmd,
        }
    ];

    tauri::Builder::default()
        .setup(|app| {

            let window = match app.get_window(HYPRSPACE_LABEL) {
                None => panic!("Unable to get hyprspace window"),
                Some(w) => w
            };

            // listen on the socket 
            start_listening_thread_on_socket(window, HYPRSPACE_SOCK_FILE_PATH);

            // listen to error events
            app.listen_global("error-message", |event| {
                react_to_error_message(event);
            });

            match app.get_cli_matches() {
                Ok(matches) => {
                    command::select_command(app, available_commands, &matches.args);

                    // temp fix
                    if matches.args.contains_key(&String::from("help")) {
                        process::exit(0);
                    }
                }
                Err(_) => {}
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_search_results,
            get_style_sheet_path,
            handle_execution_request
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");


    info!("---------- END LOG ----------");
}
