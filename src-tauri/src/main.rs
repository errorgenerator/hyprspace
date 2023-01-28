// TODO: Clean up Code
#[cfg(target_os = "linux")]
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
use tauri::{Manager, WindowBuilder, WindowUrl};

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

    let available_commands = vec![
        HyprSpaceCommand {
            cmd_name: "help".to_string(),
            cmd_function: command::help_cmd,
        },
        HyprSpaceCommand {
            cmd_name: "show-window".to_string(),
            cmd_function: command::show_window_on_startup_cmd,
        },
    ];

    // Build the main window
    tauri::Builder::default()
        .setup(|app| {
            // create the main window
            match WindowBuilder::new(app, HYPRSPACE_LABEL, WindowUrl::App("index.html".into()))
                .title(HYPRSPACE_LABEL)
                .enable_clipboard_access()
                .accept_first_mouse(true)
                .resizable(true)
                .center()
                .always_on_top(true)
                .skip_taskbar(true)
                .transparent(true)
                .decorations(false)
                .focused(true)
                .fullscreen(true)
                .visible(false)
                .build()
            {
                Err(why) => {
                    panic!("Unable to buld window!\nError: {}", why);
                }
                Ok(_s) => {}
            }

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
