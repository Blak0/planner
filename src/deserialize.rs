use serde::{Deserialize, Serialize};
use serde_yaml;

use std::{collections::HashMap, io};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]

pub struct YamlSchema {
    pub settings: Settings,
    pub tasks: HashMap<usize, Job>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Settings {
    pub start_at: String,
    pub work_start_hour: usize,
    pub work_end_hour: usize,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Job {
    pub duration: usize,
    pub description: String,
    pub prerequirements: Option<Vec<usize>>,
}

pub fn deserialize_yaml(reader: impl io::Read) -> YamlSchema {
    serde_yaml::from_reader(reader).unwrap()
}
