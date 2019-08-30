#[derive(Debug)]
pub enum NodeType {
    NONE,
    PASSAGE,
    PARAGRAPH,
    HEADING,
    CHAR,
    BOLD,
    ITALIC
}

#[derive(Debug)]
pub struct Node {
    pub node_type: NodeType,
    pub depth: i32,
    pub parent_id: usize,
    pub children: Vec<usize>,
    pub data: i32,   // for char, it's unicode
    // for heading, it's heading level
}

impl Node {
    pub fn new() -> Node {
        Node {
            node_type: NodeType::NONE,
            depth: 0,
            parent_id: 0,
            children: vec![],
            data: 0,
        }
    }
    pub fn push(&mut self, n: usize) {
        self.children.push(n);
    }
}
