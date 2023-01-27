use std::process::Command;

use crate::config::create_app_configuration;

#[tauri::command]
pub async fn handle_execution_request(exe_path: String, type_of_exe: String) -> String {
    let cached_config = create_app_configuration(None);

    if type_of_exe.eq("executable") {
        match cached_config
            .config
            .preferred_terminal_emulator
            .unwrap()
            .as_str()
        {
            "kitty" => {
                return spawn_with_kitty(exe_path.clone().as_str());
            }
            "alacritty" => {
                return spawn_with_alacritty(exe_path.clone().as_str());
            }
            "foot" => {
                return spawn_with_foot(exe_path.clone().as_str());
            }
            _ => {
                return String::from(
                    "UNKOWN TERMINAL EMULATOR!\nAVAILABLE EMULATORS ARE: ALACRITTY, FOOT, KITTY!",
                );
            }
        }
    }

    match Command::new(exe_path.clone().as_str()).spawn() {
        Err(why) => {
            error!(
                "Unable to launch application: {}\nError: {}",
                exe_path.clone(),
                why
            );
            return String::from("Error while trying to launch application");
        }
        Ok(_c) => {
            return String::from("OK");
        }
    };
}

fn spawn_with_foot(exe_path: &str) -> String {
    // Foot
    let exe_arg = exe_path.clone();
    match Command::new("foot").arg(exe_arg).spawn() {
        Err(why) => {
            error!(
                "Unable to execute command in foot: {}\nError: {}",
                exe_path.clone(),
                why
            );
            return String::from("Error in executing command");
        }
        Ok(mut c) => match c.wait() {
            Err(why) => {
                error!(
                    "Something went wrong while trying to spawn a terminal!\nError: {}",
                    why
                );
                return String::from("Error in executing command");
            }
            Ok(_s) => {
                return String::from("OK");
            }
        },
    }
}

fn spawn_with_alacritty(exe_path: &str) -> String {
    // Alacritty
    let exe_arg = exe_path.clone();
    let command_arg = "--command";
    match Command::new("alacritty")
        .arg(command_arg)
        .arg(exe_arg)
        .spawn()
    {
        Err(why) => {
            error!(
                "Unable to execute command on alacritty: {}\nError: {}",
                exe_path.clone(),
                why
            );
            return String::from("Error in executing command!");
        }
        Ok(mut c) => match c.wait() {
            Err(why) => {
                error!(
                    "Something went wrong while trying to spawn a terminal!\nError: {}",
                    why
                );
                return String::from("Error in executing command!");
            }
            Ok(_s) => {
                return String::from("OK");
            }
        },
    }
}

fn spawn_with_kitty(exe_path: &str) -> String {
    // Kitty
    let exe_arg = exe_path.clone();
    let kitty_detach_flag = "--detach";
    let kitty_start_normal = "--start-as=normal";
    match Command::new("kitty")
        .arg(kitty_detach_flag)
        .arg(kitty_start_normal)
        .arg(exe_arg)
        .spawn()
    {
        Err(why) => {
            error!(
                "Unable to execute command: {} on kitty\nError: {}",
                exe_path.clone(),
                why
            );
            return String::from("Error in executing command!");
        }
        Ok(mut c) => {
            match c.wait() {
                Err(why) => {
                    error!(
                        "Something went wrong while trying to spawn a terminal!\nError: {}",
                        why
                    );
                    return String::from("Error in executing command");
                }
                Ok(_s) => {
                    return String::from("OK");
                }
            };
        }
    }
}
