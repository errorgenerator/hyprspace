use std::collections::HashMap;

use rust_fuzzy_search::{fuzzy_search_best_n, fuzzy_search_sorted};

use crate::{applications::{self, application::Application, executable::Executable}, config::create_app_configuration};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchResult {
    pub applications: Vec<Application>,
    pub executables: Vec<Executable>,
}

#[tauri::command]
pub async fn get_search_results(
    search_term: String,
    reload_cache: bool,
) -> SearchResult {

    let cached_config = create_app_configuration(None);

    let limit = cached_config.config.application_results_limit.unwrap();

    let mut found_apps: Vec<Application> = Vec::new();
    let mut found_exes: Vec<Executable> = Vec::new();

    if !search_term.trim().is_empty() && !search_term.trim().eq("") {

        let applications = applications::load_apps_list(reload_cache);

        let app_names = applications.application_names_vec.clone();
        let app_map = applications.application_vec.clone();
        let search_term_apps = search_term.clone();

        let exe_names = applications.executable_names_vec.clone();
        let exe_map = applications.executable_vec.clone();
        let search_term_exes = search_term.clone();

        let join_handle_for_apps = std::thread::spawn(move || {
            find_applications(search_term_apps, limit, &app_names, app_map)
        });

        let join_handle_for_exes = std::thread::spawn(move || {
            find_executables(search_term_exes, limit, &exe_names, exe_map)
        });

        found_apps = match join_handle_for_apps.join() {
            Err(why) => {
                error!(
                    "Thread for finding app search results panicked!\nError: {:?}",
                    why
                );
                Vec::new()
            }
            Ok(v) => v
        };

        found_exes = match join_handle_for_exes.join() {
            Err(why) => {
                error!(
                    "Thread for finding executables search results panicked!\nError: {:?}",
                    why
                );
                Vec::new()
            },
            Ok(v) => v
        };

    }

    SearchResult{ applications: found_apps, executables: found_exes}
}

fn find_executables (
    search_term: String,
    limit: u32,
    exe_names: &Vec<String>,
    exe_map: HashMap<String, Executable>
) -> Vec<Executable> {

    let mut exe_str_vec: Vec<&str> = Vec::new();

    for name in exe_names {
        let name_owner = name;
        exe_str_vec.push(name_owner.as_str());
    }

    let mut found_exes: Vec<Executable> = Vec::new();
    let mut best_matches = fuzzy_search_best_n(search_term.as_str(), &exe_str_vec, limit as usize);

    best_matches.sort_unstable_by(|a, b| {
        match b.1.partial_cmp(&a.1) {
            None => std::cmp::Ordering::Equal,
            Some(o) => o
        }
    });

    for (name, _score) in best_matches {
        match exe_map.get(&String::from(name)) {
            None => {
                continue;
            },
            Some(e) => {
                found_exes.push(e.clone());
            }
        }
    }

    found_exes
}

fn find_applications(
    search_term: String,
    limit: u32,
    app_names: &Vec<String>,
    app_map: HashMap<String, Application>,
) -> Vec<Application> {

    let mut app_str_vec: Vec<&str> = Vec::new();

    for name in app_names {
        let name_owner = name;
        app_str_vec.push(name_owner.as_str());
    }

    let mut found_applications: Vec<Application> = Vec::new();
    let mut best_matches = fuzzy_search_sorted(search_term.as_str(), &app_str_vec);

    // sort the array
    best_matches.sort_unstable_by(|a, b| {
        match b.1.partial_cmp(&a.1) {
            None => std::cmp::Ordering::Equal,
            Some(o) => o
        }
    });

    let mut count = 0;

    for (name, _score) in best_matches {

        if count >= limit {
            break;
        }

        match app_map.get(&String::from(name)) {
            None => {
                continue;
            },
            Some(a) => {
                found_applications.push(a.clone());
                count += 1;
            }
        }
    }

    found_applications
}
