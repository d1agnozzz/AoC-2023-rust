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
        }
        // ...
        self.count += 1;
        true
    }

    // fn isert_at(&mut self, at_node: NodeHandle, data: NodeData) -> bool {
    //     if data == self.nodes[&at_node].data {
    //         return false;
    //     }
    //     if data.self_interval.low < self.nodes[&at_node].data.self_interval.low {
    //         if self.nodes[&at_node].left == 0 {}
    //     }
    // }

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
    left: NodeHandle,
    right: NodeHandle,
    parent: NodeHandle,
}

impl Node {
    fn new(data: NodeData) -> Self {
        Self {
            max_value: data.self_interval.high,
            data,
            left: 0,
            right: 0,
            parent: 0,
        }
    }
    fn new_with_parent(data: NodeData, parent: NodeHandle) -> Self {
        Self {
            max_value: data.self_interval.high,
            data,
            left: 0,
            right: 0,
            parent,
        }
    }
}
