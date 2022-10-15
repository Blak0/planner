use serde::{Deserialize, Serialize};
use serde_yaml;

use std::{collections::HashMap, error::Error, fs::File};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct YamlSchema {
    pub settings: Settings,
    pub tasks: HashMap<usize, JobNode>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Settings {
    pub start_at: String,
    pub work_start_hour: usize,
    pub work_end_hour: usize,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct JobNode {
    pub duration: usize,
    pub description: String,
    pub next: Option<Vec<usize>>,
}

pub fn deserialize_yaml(path: String) -> Result<YamlSchema, Box<dyn Error>> {
    let reader = File::open(path)?;
    Ok(serde_yaml::from_reader(reader).unwrap())
}
