use crate::{deserialize::JobNode, types::Job};
use std::collections::HashMap;

#[derive(Debug)]
struct BidirectionalGraph {
    pub(crate) data: Vec<BidirectionalJob<JobNode>>,
}

#[derive(Clone, Debug)]
pub(crate) struct BidirectionalJob<T> {
    pub(crate) id: usize,
    pub(crate) prevs: Vec<usize>,
    value: T,
}

impl BidirectionalGraph {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn from_hashmap(data: HashMap<usize, JobNode>) -> Self {
        let mut graph = Self::new();
        for (idx, job) in &data {
            let mut prevs = vec![];

            for (job_idx, job_entry) in &data {
                if job_entry.next.clone().unwrap_or(vec![]).contains(&idx) {
                    prevs.push(*job_idx);
                }
            }

            graph.data.push(BidirectionalJob {
                id: *idx,
                prevs: prevs,
                value: job.clone(),
            });
        }

        graph
    }

    pub fn topological_sort(self) -> Result<Vec<Job>, &'static str> {
        let mut result = vec![];

        let mut data = self.data;

        loop {
            let (no_prevs, rest) = Self::first_node_with_no_prevs(data);
            match no_prevs {
                Some(some_prev) => {
                    data = rest.into_iter().filter(|n| n.id != some_prev.id).collect();

                    result.push(some_prev);
                }
                None if rest.len() == 0 => break,
                None => return Err("Graph contains cycles."),
            }
        }

        Ok(result
            .into_iter()
            .map(|x| Job {
                duration: x.value.duration,
                description: x.value.description,
            })
            .collect())
    }

    fn first_node_with_no_prevs<T>(
        nodes: Vec<BidirectionalJob<T>>,
    ) -> (Option<BidirectionalJob<T>>, Vec<BidirectionalJob<T>>) {
        let mut first_no_prev = None;
        let mut rest = vec![];

        nodes.into_iter().for_each(|job| {
            if job.prevs.is_empty() && first_no_prev.is_none() {
                first_no_prev = Some(job);
            } else {
                rest.push(job);
            }
        });

        if let Some(ref p) = first_no_prev {
            for node in &mut rest {
                let new_prevs = node
                    .prevs
                    .iter()
                    .map(|val| *val)
                    .filter(|prev| prev != &p.id)
                    .collect();
                node.prevs = new_prevs;
            }
        }

        return (first_no_prev, rest);
    }
}

impl Default for BidirectionalGraph {
    fn default() -> Self {
        Self::new()
    }
}

pub(crate) fn topological_sort_jobs(
    data: HashMap<usize, JobNode>,
) -> Result<Vec<Job>, &'static str> {
    let graph = BidirectionalGraph::from_hashmap(data);

    graph.topological_sort()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let mut data = HashMap::new();
        data.insert(
            1,
            JobNode {
                duration: 1,
                description: "1".to_string(),
                next: Some(vec![2, 3]),
            },
        );
        data.insert(
            2,
            JobNode {
                duration: 1,
                description: "2".to_string(),
                next: Some(vec![4, 5]),
            },
        );
        data.insert(
            3,
            JobNode {
                duration: 1,
                description: "3".to_string(),
                next: Some(vec![4]),
            },
        );
        data.insert(
            4,
            JobNode {
                duration: 1,
                description: "4".to_string(),
                next: Some(vec![5]),
            },
        );
        data.insert(
            5,
            JobNode {
                duration: 1,
                description: "5".to_string(),
                next: Some(vec![6]),
            },
        );

        let result = topological_sort_jobs(data);
        assert!(result.is_ok());
        assert!(result.unwrap().len() == 5);
    }

    #[test]
    fn test_err_circular() {
        let mut data = HashMap::new();

        data.insert(
            1,
            JobNode {
                duration: 1,
                description: "1".to_string(),
                next: Some(vec![2]),
            },
        );
        data.insert(
            2,
            JobNode {
                duration: 1,
                description: "2".to_string(),
                next: Some(vec![3]),
            },
        );
        data.insert(
            3,
            JobNode {
                duration: 1,
                description: "3".to_string(),
                next: Some(vec![4]),
            },
        );
        data.insert(
            4,
            JobNode {
                duration: 1,
                description: "4".to_string(),
                next: Some(vec![5]),
            },
        );
        data.insert(
            5,
            JobNode {
                duration: 1,
                description: "5".to_string(),
                next: Some(vec![1]),
            },
        );

        let result = topological_sort_jobs(data);
        assert!(result.is_err());
    }
}
