#![allow(warnings)]

use std::error::Error;

use std::fs::File;

mod deserialize;
mod graph;
mod task_management;
mod topological_sort;
use crate::deserialize::{deserialize_yaml, YamlSchema};



fn main() -> Result<(), Box<dyn Error>> {
    let yaml = File::open("test_plan2.yaml")?;

    // let values: YamlSchema = deserialize_yaml(yaml);

    // println!("{:#?}", &values);

    // let lined_up_tasks = task_management::topological_sort_tasks(values.tasks);
    // let priority_sorted = task_management::sort_by_priority_descending(lined_up_tasks);

    // println!("{:#?}", lined_up_tasks);
    Ok(())
}
