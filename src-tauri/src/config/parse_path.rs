use super::create_app_configuration;

/// This function is used to parse the system path
/// in order to get access to a list of all available
/// executables (binaries) on the system.
///
/// Returns a `Result<Vec<String>, String>`
pub fn parse_path() -> Result<Vec<String>, String> {

    let cached_configuration = create_app_configuration(None);

    let path_variable = match std::env::var_os(cached_configuration.applications.path_var.unwrap()) {
        None => String::new(),
        Some(s) => String::from(s.to_string_lossy())
    };

    match path_variable.is_empty() {
        true => Err(String::from("$PATH could not be determined")),
        false => Ok(dissect_path_into_directories(path_variable))
    }
}

fn dissect_path_into_directories(output: String) -> Vec<String> {
    let mut parsed_vec: Vec<String> = Vec::new();
    let split_str = output.split(":");

    for s in split_str {
        parsed_vec.push(format!("{}/", String::from(s)));
    }

    parsed_vec
}