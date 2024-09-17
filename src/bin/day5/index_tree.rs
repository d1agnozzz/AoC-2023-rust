use nom::error::ParseError;

use super::types::ClosedInterval;
use std::collections::HashMap;
type NodeHandle = usize;
type TreeHandle = usize;

pub struct IntervalTree {
    root: Option<NodeHandle>,
    nodes: HashMap<NodeHandle, Node>,
    count: usize,
}

impl IntervalTree {
    pub fn insert(&mut self, data: NodeData) -> bool {
        if let None = self.root {
            self.root = Some(self.alloc_node(data, 0));
        } else if !self.insert_at(self.root.unwrap(), data) {
            return false;
        }
        self.count += 1;
        true
    }

    fn insert_at(&mut self, at_node: NodeHandle, data: NodeData) -> bool {
        if data == self.nodes[&at_node].data {
            return false;
        }
        if data.self_interval.low < self.nodes[&at_node].data.self_interval.low {
            if self.nodes[&at_node].left.is_none() {
                self.nodes.get_mut(&at_node).unwrap().left = Some(self.alloc_node(data, at_node));
                true
            } else {
                self.insert_at(self.nodes[&at_node].left.unwrap(), data)
            }
        } else if self.nodes[&at_node].right.is_none() {
            self.nodes.get_mut(&at_node).unwrap().right = Some(self.alloc_node(data, at_node));
            true
        } else {
            self.insert_at(self.nodes[&at_node].right.unwrap(), data)
        }
    }

    fn alloc_node(&mut self, data: NodeData, parent: NodeHandle) -> NodeHandle {
        self.nodes
            .insert(self.count, Node::new_with_parent(data, parent));
        self.nodes.len() - 1
    }
}

#[derive(PartialEq)]
struct NodeData {
    self_interval: ClosedInterval,
    other_tree: TreeHandle,
    other_interval: NodeHandle,
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
            max_value: data.self_interval.high,
            data,
            left: None,
            right: None,
            parent: None,
        }
    }
    fn new_with_parent(data: NodeData, parent: NodeHandle) -> Self {
        Self {
            max_value: data.self_interval.high,
            data,
            left: None,
            right: None,
            parent: Some(parent),
        }
    }
}
