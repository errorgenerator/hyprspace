use std::collections::HashMap;


/// Represents a desktop file entry for an application.
/// Contains a name as well as a full path to the executable or 
/// shell script (full path includes the filename, eg "/usr/bin/codium")
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Application {
    pub name: String,
    pub full_path: String,
    pub path_to_icon: String,
}

/// Builds both the vector for the names of all available desktop entries as
/// well as a HashMap for the executables associated with those applications
/// 
/// Note that the names vector `Vec<String>` is sorted alphabetically to ease
/// searching through it later on.
fn build_applications() -> (Vec<String>, HashMap<String, Application>) {
    let mut names_vec: Vec<String> = Vec::new();
    let mut paths_map: HashMap<String, Application> = HashMap::new();



    (names_vec, paths_map)
}