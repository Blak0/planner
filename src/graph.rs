use std::{fmt, ops::Index};

pub struct DirectedNode<T> {
    pub value: T,
    pub(crate) next: Vec<usize>,
    pub(crate) prevs: Vec<usize>,
}

impl<T> DirectedNode<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            next: vec![],
            prevs: vec![],
        }
    }
}

pub struct DirectedGraph<T> {
    data: Vec<DirectedNode<T>>,
}

impl<T> DirectedGraph<T> {
    pub fn new(data: Vec<T>) -> Self {
        let packed_data = data.into_iter().map(DirectedNode::new).collect();
        Self { data: packed_data }
    }

    pub fn add(&mut self, item: T) {
        self.data.push(DirectedNode::new(item));
    }

    pub fn add_node(&mut self, node: DirectedNode<T>) {
        self.data.push(node);
    }

    pub fn get(&self, item: T) -> Option<&DirectedNode<T>>
    where
        T: PartialEq,
    {
        self.data.iter().find(|x| x.value == item)
    }

    pub fn get_idx(&self, idx: usize) -> Option<&DirectedNode<T>> {
        self.data.get(idx)
    }

    pub fn get_nodes_with_no_prevs(&self) -> Vec<&DirectedNode<T>> {
        let mut results = Vec::new();
        for node in &self.data {
            if node.prevs.is_empty() {
                results.push(node);
            }
        }
        results
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<T> Default for DirectedGraph<T> {
    fn default() -> Self {
        Self::new(vec![])
    }
}

impl<T> IntoIterator for DirectedGraph<T> {
    type Item = DirectedNode<T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[inline]
    fn get_empty_graph<T>() -> DirectedGraph<T> {
        DirectedGraph::<T>::default()
    }
    #[test]
    fn test_default_is_empty() {
        let g = get_empty_graph::<()>();
        assert!(g.len() == 0);
    }

    #[test]
    fn test_default_then_add() {
        let mut g = get_empty_graph();
        g.add(3);
        g.add(15);

        assert!(g.len() == 2)
    }

    #[test]
    fn test_new() {
        let g = DirectedGraph::new(vec![1, 2, 3, 4]);
        assert!(g.len() == 4);
    }

    #[test]
    fn test_get() {
        let mut g = get_empty_graph();
        g.add(3);
        assert!(g.get(3).is_some());
        assert!(g.get(5).is_none());
    }

    #[test]
    fn test_add_node() {
        let mut g = get_empty_graph();
        g.add_node(DirectedNode {
            value: 3,
            next: vec![],
            prevs: vec![],
        });

        g.add_node(DirectedNode { value: 16, next: vec![], prevs: vec![0] })
    }

    #[test]
    fn test_link_node() {
        let mut g = get_empty_graph();
        g.add(3);
        g.add_node(DirectedNode { value: 16, next: vec![], prevs: vec![0] })

    }
}
