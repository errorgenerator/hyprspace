// TODO: Clean up Code
// TODO: Change this config
// TODO: gotta use a Boyer-Moore algorithm for searching applications.
#![cfg_attr(
    all(not(debug_assertions), target_os = "linux"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate log;

mod applications;
mod command;
mod config;
mod events;
mod socket;

use std::process;

use events::{react_to_error_message, search::get_search_results};
use socket::start_listening_thread_on_socket;
use tauri::Manager;

use crate::{
    applications::load_apps_list,
    command::HyprSpaceCommand,
    config::{
        create_app_configuration, defaults::HYPRSPACE_LABEL, load_style::get_style_sheet_path,
    },
    events::execute::handle_execution_request,
};

// ### Setup ###
fn main() {
    // start up logging framework
    env_logger::init();
    info!("---------- BEGIN LOG ----------");

    // initialize the config cache
    create_app_configuration(None);

    // initialize the application cache
    load_apps_list(false);

    let available_commands = vec![HyprSpaceCommand {
        cmd_name: "help".to_string(),
        cmd_function: command::help_cmd,
    }];

    tauri::Builder::default()
        .setup(|app| {
            let window = match app.get_window(HYPRSPACE_LABEL) {
                None => panic!("Unable to get hyprspace window"),
                Some(w) => w,
            };

            // listen on the socket
            start_listening_thread_on_socket(window);

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
