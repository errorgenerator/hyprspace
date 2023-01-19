#[tauri::command]
pub async fn get_search_results(search_term: String) -> String {
    // TODO Implement rendering logic
    format!("PLACEHOLDER VALUE! {}", search_term)
}