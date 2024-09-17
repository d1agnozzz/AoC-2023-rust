use nom::error::ParseError;

use super::types::ClosedInterval;
use std::collections::HashMap;
type NodeHandle = usize;

pub struct IntervalTree {
    root: Option<NodeHandle>,
    nodes: HashMap<NodeHandle, Node>,
    count: usize,
}

impl IntervalTree {
    pub fn node_count(&self) -> usize {
        assert!(self.count != 0 || self.root.is_some());
        self.count
    }
    pub fn insert(&mut self, data: NodeData) -> bool {
        match self.root {
            None => {
                self.root = Some(self.alloc_node(data, 0));
            }
            Some(n) => {
                let inserted = self.insert_at(n, data);
                return inserted != n;
            }
        }
        self.count += 1;
        true
    }

    fn insert_at(&mut self, at_node: NodeHandle, data: NodeData) -> NodeHandle {
        if data == self.nodes[&at_node].data {
            return at_node;
        }

        if data.original_interval.low < self.nodes[&at_node].data.original_interval.low {
            match self.nodes[&at_node].left {
                None => {
                    self.nodes.get_mut(&at_node).unwrap().left =
                        Some(self.alloc_node(data, at_node));
                }
                Some(n) => {
                    self.insert_at(n, data);
                }
            };
        }

        match self.nodes[&at_node].right {
            None => {
                self.nodes.get_mut(&at_node).unwrap().right = Some(self.alloc_node(data, at_node));
            }
            Some(n) => {
                self.insert_at(n, data);
            }
        };

        if self.nodes[&at_node].max_value < data.original_interval.high {
            self.nodes.get_mut(&at_node).unwrap().max_value = data.original_interval.high;
        }

        at_node
    }

    fn alloc_node(&mut self, data: NodeData, parent: NodeHandle) -> NodeHandle {
        self.nodes
            .insert(self.count, Node::new_with_parent(data, parent));
        self.nodes.len() - 1
    }
    fn find_node(&self, from_node: NodeHandle, data: NodeData) -> Option<NodeHandle> {
        if from_node == 0 {
            return None;
        }

        let node = &self.nodes.get(&from_node)?;
        if node.data == data {
            Some(from_node)
        } else if node
            .data
            .original_interval
            .overlaps(&data.original_interval)
        {
            match node.left {
                None => None,
                _ => self.find_node(node.left?, data),
            }
        } else {
            match node.right {
                Node => None,
                _ => self.find_node(node.right?, data),
            }
        }
    }
    // fn remove(&mut self, data: NodeData) -> bool {
    // if let Some(node) = self.find_node
    // }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct NodeData {
    original_interval: ClosedInterval,
    mapped_interval: ClosedInterval,
}

struct Node {
    data: NodeData,
    max_value: usize,
    left: Option<NodeHandle>,
    right: Option<NodeHandle>,
    parent: Option<NodeHandle>,
}

impl Node {
    fn new(data: NodeData) -> Self {
        Self {
            max_value: data.original_interval.high,
            data,
            left: None,
            right: None,
            parent: None,
        }
    }
    fn new_with_parent(data: NodeData, parent: NodeHandle) -> Self {
        Self {
            max_value: data.original_interval.high,
            data,
            left: None,
            right: None,
            parent: Some(parent),
        }
    }
}
