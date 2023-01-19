pub mod application;
pub mod executable;

use std::{
    collections::HashMap, 
    ffi::OsString, 
    path::PathBuf, fs
};

use application::Application;
use executable::Executable;

use crate::config::parse_path::parse_path;

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

    let paths = match fs::read_dir(path) {
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


/// Builds both the vector for all available executables on $PATH as well as
/// a HasMap for the executables which contains the name of the executable as
/// key and an Executable Struct providing a path to the executable.
/// 
/// Note that the names vector `Vec<String>` is sorted alphabetically to ease
/// searching through it later on.
fn build_executables() -> (Vec<String>, HashMap<String, Executable>) {
    let mut names_vec: Vec<String> = Vec::new(); 
    let mut paths_map: HashMap<String, Executable> = HashMap::new();

    let dirs_on_path = match build_executables_dirs_list() {
        None => {
            error!("Unable to get a list of the directories on $PATH!\nCheck the logs!");
            return (names_vec, paths_map);
        },
        Some(v) => v
    };


    // sort the vector
    names_vec.sort_unstable();
    (names_vec, paths_map)
}

fn build_executables_dirs_list() -> Option<Vec<PathBuf>> {
    match parse_path() {
        Err(why) => {
            error!("{}", why);
            None
        }
        Ok(v) => Some(convert_to_paths(v)),
    }
}

fn convert_to_paths(string_vec: Vec<String>) -> Vec<PathBuf> {
    let mut r_vec: Vec<PathBuf> = Vec::new();
    for s in string_vec {
        let os_vers = OsString::from(s);
        r_vec.push(PathBuf::from(os_vers));
    }
    r_vec
}
