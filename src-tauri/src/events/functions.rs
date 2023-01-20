use cached::proc_macro::cached;

use crate::applications::{self, ApplicationList, application::Application, executable::Executable};

#[tauri::command]
pub async fn get_search_results(search_term: String,limit: u32, reload_cache: bool) -> String {
    let applications = applications::load_apps_list(reload_cache);

    if search_term.is_empty() {
        return String::from("");
    }

    // TODO Implement rendering logic
    format!("{}<br>{:?}", applications.app_count, applications.application_names_vec)
}

// TODO: multi-threaded search for slight performance increase

fn render_found_applications(limit: u32, app_vec: Vec<Application>) -> String {
    todo();
}

fn render_found_cmds(limit: u32, app_vec: Vec<Executable>) -> String {
    todo();
}

fn find_applications_by_search_term(limit: u32, search_term: String, application_list: ApplicationList) -> Vec<Application> {
    let application_names = application_list.application_names_vec;
    todo();
}