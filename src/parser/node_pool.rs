use crate::parser::node::Node;

#[derive(Debug)]
pub struct NodePool {
    nodes: Vec<Node>,
}

impl NodePool {
    pub fn new() -> NodePool {
        NodePool {
            nodes: vec![],
        }
    }
    pub fn get(&self, n: usize) -> &Node {
        &self.nodes[n]
    }
    pub fn get_mut(&mut self, n: usize) -> &mut Node {
        &mut self.nodes[n]
    }
    pub fn push(&mut self, n: Node) -> usize {
        self.nodes.push(n);
        self.nodes.len() - 1
    }
    /// creates a new node and returns its index
    pub fn new_node(&mut self) -> usize {
        self.nodes.push(Node::new());
        self.nodes.len() - 1
    }
}
