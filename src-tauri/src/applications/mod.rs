pub mod application;
pub mod executable;

use std::{
    collections::HashMap, 
    ffi::OsString, 
    path::PathBuf, fs
};

use application::Application;
use executable::Executable;


/// This struct holds all available Information 
/// regarding Applications and executables (commands)
/// available on the system.
/// 
/// As building this struct is very resource intensive,
/// it will only be built once on startup of the application and
/// will then only refresh its list of applications on a change in the
/// watched directories ()
pub struct ApplicationList {
    pub app_count: u32,
    pub exe_count: u32,
    pub application_names_vec: Vec<String>,
    pub executable_names_vec: Vec<String>,
    pub application_vec: HashMap<String, Application>,
    pub executable_vec: HashMap<String, Executable>,
}

/// This function simply lists all file in a specific directory passed to it
/// as a PathBuf.
/// Note that, if a entry is, for whatever reason, unreadable it will simply be
/// skipped.
/// 
/// Returns a tuple of `(PathBuf, Vec<String>).
/// This is needed in order to preserve the full path to an executable for later.
fn list_dir_contents(path: PathBuf) -> (PathBuf, Vec<String>) {
    let mut contents: Vec<String> = Vec::new();

    let paths = match fs::read_dir(&path) {
        Err(why) => {
            error!(
                "Unable to list contents of directory: {}!\nError: {}",
                path.display(),
                why
            );
            return (path, contents);
        },
        Ok(p) => p
    };

    for p in paths {
        match p {
            Err(why) => {
                error!("Unable to read entry!\nError: {}", why);
                continue; // simply skip the entry
            },
            Ok(e) => {

                let file_name = e.file_name().to_string_lossy().to_string(); // TODO: check if this has an effect
                contents.push(file_name);
            }
        }
    }

    contents.sort_unstable();
    (path, contents)
}

// Converts a String to a PathBuf
fn convert_to_paths(string_vec: Vec<String>) -> Vec<PathBuf> {
    let mut r_vec: Vec<PathBuf> = Vec::new();
    for s in string_vec {
        let os_vers = OsString::from(s);
        r_vec.push(PathBuf::from(os_vers));
    }
    r_vec
}
