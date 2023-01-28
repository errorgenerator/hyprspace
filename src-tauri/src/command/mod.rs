//TODO: Documentation!
use std::collections::HashMap;

use tauri::{api::cli::ArgData, App};



pub struct HyprSpaceCommand {
    pub cmd_name: String,
    pub cmd_function: fn(app: &mut App, data: &ArgData) -> String,
}

impl HyprSpaceCommand {
    fn execute(&mut self, app: &mut App, data: &ArgData) -> String {
        (self.cmd_function)(app, data)
    }
}

pub fn help_cmd(_app: &mut App, data: &ArgData) -> String {
    data.value.to_string()
}


pub fn show_window_on_startup_cmd(app: &mut App, _data: &ArgData) -> String {
    use tauri::Manager;
    use crate::config::defaults;
    let window = app.get_window(defaults::HYPRSPACE_LABEL);

    match window {
        None => {
            panic!("Unable to get the window on startup, something is fucky here...");
        },
        Some(w) => {
            match w.show() {
                Err(why) => {
                    panic!("Was able to get the window on startup, but an Error occurred!\nError: {}", why);
                },
                Ok(()) => {}
            }
        }
    }

    String::from("Started Application with window shown on startup")
}

pub fn select_command(
    app: &mut App,
    available_commands: Vec<HyprSpaceCommand>,
    args: &HashMap<String, ArgData>,
) {
    let mut collected_output = String::from("");

    for mut command in available_commands {
        if args.contains_key(&command.cmd_name) {
            match args.get(&command.cmd_name) {
                None => continue,
                Some(d) => {
                    let cmd_output = command.execute(app, d);
                    collected_output = format!("{}\n{}", collected_output, cmd_output);
                }
            };
        }
    }
    println!(
        "{}",
        collected_output.replace("\\n", "\n").replace("\"", "")
    );
}
