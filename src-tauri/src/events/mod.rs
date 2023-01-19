use tauri::Event;

use crate::events::messages::ErrorEvent;

pub mod messages;
pub mod functions;

pub trait HasComponentOfOrigin {
    fn get_component_of_origin(&self) -> String;
}

pub trait HasMessage {
    fn get_message(&self) -> String;
}

pub fn react_to_error_message(event: Event) {
    let payload = match event.payload() {
        None => "",
        Some(s) => s,
    };

    let error_message_struct: ErrorEvent = match serde_json::from_str(payload) {
        Err(why) => panic!(
            "Tried to parse an Error Message, but it failed!\nOriginal Message: {}\nError: {}",
            payload, why
        ),
        Ok(st) => st,
    };

    error!(
        "Received an Error Message from the FrontEnd!\nError: {}\nMessage: {}\nComponent: {}",
        error_message_struct.error,
        error_message_struct.message,
        error_message_struct.component_of_origin
    );
}
