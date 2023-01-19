use super::{HasComponentOfOrigin, HasMessage};


/**
 * Emitted if there is need to reload the System - Application list
 * {
 *   reload: boolean,
 *   message: "string",
 *   component_of_origin: "ComponentName"
 * }
 */
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReloadRequest {
    pub reload: bool,
    pub message: String,
    pub component_of_origin: String,
}

impl HasComponentOfOrigin for ReloadRequest {
    fn get_component_of_origin(&self) -> String {
        self.component_of_origin.clone()
    }
}

impl HasMessage for ReloadRequest {
    fn get_message(&self) -> String {
        self.message.clone()
    }
}

/**
 * Emitted on UserInput in Searchbar
 * {
 *   message: "The UserInput",
 *   component_of_origin: "ComponentName"
 * }
 */
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchInputEvent {
    pub message: String,
    pub component_of_origin: String,
}

impl HasMessage for SearchInputEvent {
    fn get_message(&self) -> String {
        self.message.clone()
    }
}

impl HasComponentOfOrigin for SearchInputEvent {
    fn get_component_of_origin(&self) -> String {
        self.component_of_origin.clone()
    }
}

/**
 * Emitted if an error occurs
 * {
 *   error: "ErrorMessage",
 *   message: "An Error occurred in component: ComponentName ==> Check the Logs. Error was: ErrorMessage",
 *   component_of_origin: "ComponentName"
 * }
 */
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErrorEvent {
    pub error: String,
    pub message: String,
    pub component_of_origin: String,
}

impl HasComponentOfOrigin for ErrorEvent {
    fn get_component_of_origin(&self) -> String {
        self.component_of_origin.clone()
    }
}

impl HasMessage for ErrorEvent {
    fn get_message(&self) -> String {
        self.message.clone()
    }
}


/**
 * Emitted on Input Change
 * {
 *   message: "The Users Input",
 *   component_of_origin: "ComponentName"
 * }
 */
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InputEvent {
    pub message: String,
    pub component_of_origin: String,
}

impl HasComponentOfOrigin for InputEvent {
    fn get_component_of_origin(&self) -> String {
        self.component_of_origin.clone()
    }
}

impl HasMessage for InputEvent {
    fn get_message(&self) -> String {
        self.message.clone()
    }
}

/**
 * Emitted on Startup, tells the frontend to load the stylesheet if present
 * {
 *   message: "/the/path/to/the/style.css"
 * }
 */
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoadStyleSheetMessage {
    message: String,
}

impl HasMessage for LoadStyleSheetMessage {
    fn get_message(&self) -> String {
        self.message.clone()
    }
}