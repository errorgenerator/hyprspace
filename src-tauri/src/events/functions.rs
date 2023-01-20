use cached::proc_macro::cached;

use crate::applications::{self, ApplicationList, application::Application};

#[tauri::command]
pub async fn get_search_results(search_term: String, reload_cache: bool) -> String {
    let applications = applications::load_apps_list(reload_cache);

    if search_term.is_empty() {
        return String::from("");
    }

    // TODO Implement rendering logic
    format!("{}<br>{:?}", applications.app_count, applications.application_names_vec)
}

//fn find_applications_by_search_term(search_term: String, application_list: ApplicationList) -> Vec<Application> {
//
//}