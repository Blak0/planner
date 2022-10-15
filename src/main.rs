use std::error::Error;

mod deserialize;
mod graph_topology;
mod task_management;
mod types;
use crate::deserialize::{deserialize_yaml, YamlSchema};

fn main() -> Result<(), Box<dyn Error>> {
    let values: YamlSchema = deserialize_yaml("yaml_tests/test_plan2.yaml".to_string())?;

    let lined_up_tasks = graph_topology::topological_sort_jobs(values.tasks);

    println!("{:#?}", lined_up_tasks);
    Ok(())
}
