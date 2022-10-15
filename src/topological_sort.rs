// use std::collections::{HashMap, VecDeque};

// use crate::{deserialize::Job, graph};

// use itertools::Itertools;

// pub fn get_non_dependant_nodes(tasks: HashMap<usize, Job>) -> Vec<Job>{
//     for (idx, task) in &tasks {
//         if task.prerequirements
//     }
// }

// pub fn line_up_tasks(tasks: HashMap<usize, Job>) -> Vec<Job> {
//     let mut wip_tasks = tasks.clone();

//     // for (idx, task) in &tasks {
//     //     if let Some(ref prereqs) = task.prerequirements {
//     //         for p in prereqs {
//     //             wip_tasks
//     //                 .get_mut(p)
//     //                 .expect(&format!(
//     //                     "Task {} has a prerequisite {} which doesn't exist.",
//     //                     idx, p
//     //                 ))
//     //                 // .priority += task.priority;
//     //         }
//     //     }
//     // }

//     wip_tasks.into_iter().map(|(_, job)| job).collect_vec()
// }
