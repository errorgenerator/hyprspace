use std::{collections::HashMap, path::PathBuf};

use cached::proc_macro::cached;

use crate::config::{parse_path::parse_path};

use super::{list_dir_contents, convert_to_paths};


/// Represents an executable in any of the directories on $PATH
/// This could either be a shell command or a full blown executable
/// Contains a name for the file itself
/// and a full path to the file (including the filename, eg "/usr/bin/alacritty")
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Executable {
    pub name: String,
    pub full_path: String,
}


/// Builds both the vector for all available executables on $PATH as well as
/// a HasMap for the executables which contains the name of the executable as
/// key and an Executable Struct providing a path to the executable.
/// 
/// Note that the names vector `Vec<String>` is sorted alphabetically to ease
/// searching through it later on.
#[cached(size=1)]
pub fn build_executables() -> (Vec<String>, HashMap<String, Executable>) {

    let mut names_vec: Vec<String> = Vec::new(); 
    let mut paths_map: HashMap<String, Executable> = HashMap::new();

    let dirs_on_path = match build_executables_dirs_list() {
        None => {
            error!("Unable to get a list of the directories on $PATH!\nCheck the logs!");
            return (names_vec, paths_map);
        },
        Some(v) => v
    };

    for d in dirs_on_path {
        match d.exists() {
            false => continue,
            true => {
                let (buff, contents) = list_dir_contents(d);
                for exe in contents {
                    let dir_string = buff.as_os_str().to_string_lossy().to_string();
                    let full_path = format!("{}/{}", dir_string, exe);
                    let executable = Executable{ name: exe.clone(), full_path: full_path.clone() };
                    names_vec.push(exe.clone());
                    paths_map.insert(exe, executable);
                }
            }
        }
    }
    // sort the vector
    names_vec.sort_unstable();
    (names_vec, paths_map)
}

// Builds a Vec of Paths to all directories on $PATH
fn build_executables_dirs_list() -> Option<Vec<PathBuf>> {
    match parse_path() {
        Err(why) => {
            error!("{}", why);
            None
        }
        Ok(v) => Some(convert_to_paths(v)),
    }
}