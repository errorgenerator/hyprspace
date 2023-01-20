use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf
};

use crate::APPLICATION_DESKTOP_FILES_DIRECTORY;

use super::list_dir_contents;

/// Represents a desktop file entry for an application.
/// Contains a name as well as a full path to the executable or
/// shell script (full path includes the filename, eg "/usr/bin/codium")
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Application {
    pub name: String,
    pub exe: String,
    pub path_to_icon: String,
}

/// Builds both the vector for the names of all available desktop entries as
/// well as a HashMap for the executables associated with those applications
///
/// Note that the names vector `Vec<String>` is sorted alphabetically to ease
/// searching through it later on.
pub fn build_applications() -> (Vec<String>, HashMap<String, Application>) {
    let mut names_vec: Vec<String> = Vec::new();
    let mut paths_map: HashMap<String, Application> = HashMap::new();

    let app_dir_path_buf = PathBuf::from(APPLICATION_DESKTOP_FILES_DIRECTORY);

    let (appdir, mut contents) = list_dir_contents(app_dir_path_buf);
    let appdir_string = appdir.as_os_str().to_string_lossy();

    for file_path in contents.clone() {
        // must be a subdir
        if !file_path.ends_with(".desktop") && !file_path.ends_with(".list") && !file_path.ends_with(".cache") {
            let subdir_string = format!("{}/{}", appdir_string.clone(), file_path.clone());
            let (_subdir, subdir_contents) = list_dir_contents(PathBuf::from(subdir_string));
            for con in subdir_contents {
                let appendix = format!("{}/{}", file_path, con);
                contents.push(appendix);
            }
        }
    }

    for file in contents {

        // skip
        if !file.ends_with(".desktop") {
            continue;
        }

        let full_path = format!("{}{}",appdir_string.clone(), file.clone());
        let full_path_path_buf = PathBuf::from(full_path);
        let (app_name, app_exe, app_icon) = match parse_desktop_file(full_path_path_buf) {
            None => {
                continue;
            },
            Some((name, exe, icon)) => (name, exe, icon)
        };
        names_vec.push(app_name.clone());
        let icon = get_icon_path_string(app_icon);
        let application = create(app_name.clone(), app_exe.clone(), icon.clone());
        paths_map.insert(app_name, application);
    }

    (names_vec, paths_map)
}

fn parse_desktop_file(path: PathBuf) -> Option<(String, String, String)> {
    let mut app_name = String::new();
    let mut app_icon = String::new();
    let mut app_exe = String::new();
    let file = match File::open(path.clone()) {
        Err(why) => {
            error!(
                "Unable to open file: {}\nError: {}",
                path.as_os_str().to_string_lossy(),
                why
            );
            return None;
        }
        Ok(f) => f,
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Err(why) => {
                error!(
                    "Unable to read line in file: {}\nError: {}",
                    path.as_os_str().to_string_lossy(),
                    why
                );
                return None;
            },
            Ok(ls) => {

                if ls.starts_with("[") || ls.starts_with(" ") {
                    continue;
                }

                if ls.starts_with("Name=") {
                    app_name.push_str(ls.clone().replace("Name=", "").as_str());
                }

                if ls.starts_with("Exec=") {
                    app_exe.push_str(ls.clone().replace("Exec=", "").as_str());
                }

                if ls.starts_with("Icon=") {
                    app_icon.push_str(ls.clone().replace("Icon=", "").as_str());
                }
            }
        }
    }

    return Some((app_name, app_exe, app_icon));
}

fn get_icon_path_string(name: String) -> String {
    super::icons::get_icon_path_for_application(name)
}


/// Create an Application struct
fn create(name: String, exe: String, path_to_icon: String) -> Application {
    Application {
        name: name,
        exe: exe,
        path_to_icon: path_to_icon,
    }
}