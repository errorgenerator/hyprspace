#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Application {

}

pub struct ApplicationList {
    pub app_count: u32,
    pub application_vec: Vec<Application>,
}