//TODO: Documentation!
use std::thread::JoinHandle;
use std::{
    io::{BufRead, BufReader, Write},
    os::unix::net::UnixListener,
    path::Path,
};

use tauri::Window;

use crate::config::create_app_configuration;
use crate::config::defaults::{HYPRSPACE_ERR_SOCK_RESPONSE, HYPRSPACE_OK_SOCK_RESPONSE};

pub fn start_listening_thread_on_socket(window: Window) -> JoinHandle<()> {
    start_listen_thread(window)
}

fn start_listen_thread(window: Window) -> JoinHandle<()> {
    let join_handle_for_thread = std::thread::spawn(move || {
        let cached_config = create_app_configuration(None);
        let path_to_socket = cached_config.socket.socket_file_path.unwrap().clone();
        listen_to_unix_socket(window, Path::new(&path_to_socket));
    });

    join_handle_for_thread
}

fn remove_socket_file_if_exists_beforehand(path: &Path) -> bool {
    match std::fs::remove_file(path) {
        Err(why) => {
            error!(
                "Unable to remove socket file: {}!\nError: {}",
                path.display(),
                why
            );
            false
        }
        Ok(()) => true,
    }
}

fn remove_socket_file_after_thread_finish(path: &Path) -> bool {
    match std::fs::remove_file(path) {
        Err(why) => {
            error!(
                "Unable to remove socket file: {}!\nError: {}",
                path.display(),
                why
            );
            false
        }
        Ok(()) => true,
    }
}

fn listen_to_unix_socket(window: Window, path_to_socket: &Path) {
    let mut thread_continue = true;

    remove_socket_file_if_exists_beforehand(path_to_socket);

    let listener = match UnixListener::bind(path_to_socket) {
        Err(why) => {
            panic!("Unable to create a socket to listen on!\nError: {}", why);
        }
        Ok(listen) => listen,
    };

    while thread_continue {
        match listener.accept() {
            Err(why) => {
                error!(
                    "Failed to accept connection on socket: {}!\nError: {}",
                    path_to_socket.display(),
                    why
                );
                thread_continue = false;
            }
            Ok((mut socket, addr)) => {
                info!("A new client connected!\n{:?} - {:?}", socket, addr);

                match socket.write_all(b"CONNECTED") {
                    Err(why) => {
                        error!(
                            "Failed to write to socket file: {}!\nError: {}",
                            path_to_socket.display(),
                            why
                        );
                    }
                    Ok(()) => {}
                }

                let mut response = String::new();
                let socket_handle = match socket.try_clone() {
                    Err(why) => {
                        panic!("Tried creating a handle for socket: {}, but was unable to do so!\nError: {}", path_to_socket.display(), why);
                    }
                    Ok(s) => s,
                };
                let mut conn = BufReader::new(socket_handle);

                match conn.read_line(&mut response) {
                    Err(why) => {
                        error!("Unable to read message from socket!\nError: {}", why);
                    }
                    Ok(_u) => {
                        info!("Client: {:?} --> Message: {}", addr, response);

                        match socket.write_all(
                            parse_command(&window, response.replace("\n", "")).as_bytes(),
                        ) {
                            Err(why) => {
                                error!("Unable to send response to command!\nError: {}", why);
                            }
                            Ok(()) => {}
                        }
                    }
                }
            }
        }
    }
    remove_socket_file_after_thread_finish(path_to_socket);
}

fn show_window(window: &Window) -> String {
    match window.show() {
        Err(why) => {
            error!(
                "Unable to set Window visible!\nWindow: {}\nError: {}",
                window.label(),
                why
            );
            return String::from(HYPRSPACE_ERR_SOCK_RESPONSE);
        }
        Ok(()) => {
            return String::from(HYPRSPACE_OK_SOCK_RESPONSE);
        }
    }
}

fn hide_window(window: &Window) -> String {
    match window.hide() {
        Err(why) => {
            error!(
                "Unable to set Window hidden!\nWindow: {}\nError: {}",
                window.label(),
                why
            );
            return String::from(HYPRSPACE_ERR_SOCK_RESPONSE);
        }
        Ok(()) => {
            return String::from(HYPRSPACE_OK_SOCK_RESPONSE);
        }
    }
}

fn parse_command(window: &Window, command: String) -> String {
    match command.as_str() {
        "TOGGLE" => {
            let is_window_visible_atm = match window.is_visible() {
                Err(why) => {
                    error!(
                        "Unable to determine Window visibility state!\nWindow: {}\nError: {}",
                        window.label(),
                        why
                    );
                    false
                }
                Ok(b) => b,
            };

            match is_window_visible_atm {
                false => {
                    return show_window(window);
                }
                true => {
                    return hide_window(window);
                }
            }
        }
        "SHOW" => {
            return show_window(window);
        }
        "HIDE" => {
            return hide_window(window);
        }
        _ => {
            warn!("A client sent a request on the socket, but I was unable to understand what he said!\nMessage was: {}", command);
            return String::from(HYPRSPACE_ERR_SOCK_RESPONSE);
        }
    }
}
