// TODO: Clean up Code
// TODO: Change this config
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
    functions::get_search_results
};
use socket::start_listening_thread_on_socket;
use tauri::Manager;

use crate::{
    command::HyprSpaceCommand, 
    config::load_style::get_style_sheet_path
};


// ### GLOBALS ###

// hyprspace main window label 
static HYPRSPACE_LABEL: &str = "hyprspace";
static HYPRSPACE_OK_SOCK_RESPONSE: &str = "OK";
static HYPRSPACE_ERR_SOCK_RESPONSE: &str = "ERR";
// static HYPRSPACE_SHOW_SOCK_COMMAND: &str = "SHOW";
// static HYPRSPACE_HIDE_SOCK_COMMAND: &str = "HIDE";
static HYPRSPACE_SOCK_FILE_PATH: &str = "/tmp/hyprspace.sock";
static HYPRSPACE_CONFIG_DIR_NAME: &str = "hyprspace";
// static HYPRSPACE_CONFIG_FILE_NAME: &str = "hyprspace.toml";
static HYPRSPACE_STYLE_FILE_NAME: &str = "style.css";

// ### Setup ###
fn main() {

    // start up logging framework
    env_logger::init();

    info!("---------- BEGIN LOG ----------");

    let available_commands = vec![
        HyprSpaceCommand {
            cmd_name: "help".to_string(),
            cmd_function: command::help_cmd,
        },
        HyprSpaceCommand {
            cmd_name: "show-window".to_string(),
            cmd_function: command::show_window_cmd,
        },
    ];

    tauri::Builder::default()
        .setup(|app| {

            let window = match app.get_window(HYPRSPACE_LABEL) {
                None => panic!("Unable to get hyprspace window"),
                Some(w) => w
            };

            start_listening_thread_on_socket(window, HYPRSPACE_SOCK_FILE_PATH);

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
            get_style_sheet_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");


    info!("---------- END LOG ----------");
}
