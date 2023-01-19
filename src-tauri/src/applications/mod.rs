pub mod application;
pub mod executable;

use std::collections::HashMap;

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


pub fn build_executables_list() {
    match parse_path() {
        Err(why) => {
            println!("{}", why);
        },
        Ok(v) => {
            for s in v {
                println!("{}", s);
            }
        }
    }
}