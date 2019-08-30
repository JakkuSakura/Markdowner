use crate::parser::node::{NodeType, Node};
use crate::parser::node_pool::NodePool;
use std::thread::current;

#[derive(Debug)]
pub struct Parser<'a> {
    text: &'a str,
    text_index: usize,

    node_current: usize,
    node_root: usize,
    node_pool: NodePool,
}

impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Self {
        let mut parser = Parser {
            text: s,
            text_index: 0,
            node_current: 0,
            node_pool: NodePool::new(),
            node_root: 0,
        };
        let n = parser.node_pool.new_node();
        parser.node_pool.get_mut(n).node_type = NodeType::PASSAGE;
        parser
    }
    pub fn current(&mut self) -> &mut Node {
        self.node_pool.get_mut(self.node_current)
    }
    pub fn checkout(&mut self) -> char {
        match self.text.chars().nth(self.text_index) {
            Some(ch) => ch,
            None => '\u{ffff}'
        }
    }
    pub fn char(&mut self) -> bool {
        let ch = self.checkout();
        if ch != '\u{ffff}' {
            self.text_index += 1;
            let node = Node {
                node_type: NodeType::CHAR,
                depth: self.current().depth + 1,
                parent_id: self.node_current,
                children: vec![],
                data: ch as i32,
            };
            let n = self.node_pool.push(node);
            self.current().push(n);
            return true;
        }
        return false;
    }
    pub fn italic(&mut self) -> bool {
        if self.checkout() == '*' {
            self.text_index += 1;

            let current = self.node_current;
            let depth = self.current().depth;
            let new_node = self.node_pool.push(Node {
                node_type: NodeType::ITALIC,
                depth: depth + 1,
                parent_id: current,
                children: vec![],
                data: 0,
            });

            self.node_current = new_node;
            loop {
                if self.bold() {}

                let ch = self.checkout();
                if ch == '\n' { break; }

                if ch == '*' {
                    self.text_index += 1;

                    self.node_current = current;
                    self.current().push(new_node);
                    return true;
                }
                if !self.char() {
                    break;
                }
            }
        }
        false
    }
    pub fn bold(&mut self) -> bool {
        if self.checkout() == '*' {
            self.text_index += 1;
            if self.checkout() == '*' {
                self.text_index += 1;

                let current = self.node_current;
                let depth = self.current().depth;
                let new_node = self.node_pool.push(Node {
                    node_type: NodeType::BOLD,
                    depth: depth + 1,
                    parent_id: current,
                    children: vec![],
                    data: 0,
                });

                self.node_current = new_node;
                loop {
                    let ch = self.checkout();
                    if ch == '\n' { break; }

                    if ch == '*' {
                        self.text_index += 1;
                        if self.checkout() == '*' {
                            self.text_index += 1;

                            self.node_current = current;
                            self.current().push(new_node);
                            return true;
                        } else {
                            self.text_index -= 1;
                        }
                    }
                    if self.italic() {} else if !self.char() {
                        break;
                    }
                }
            }
            self.text_index -= 1;
        }
        false
    }
    pub fn text_to_newline(&mut self) {
        loop {
            if self.bold() {} else if self.italic() {} else if self.checkout() == '\n' { break; } else if self.char() {} else { break; }
        }
    }
    pub fn paragraph(&mut self, from_begin: bool) -> bool {
        self.text_index += 1;

        let current = self.node_current;
        let depth = self.current().depth;
        let new_node = self.node_pool.push(Node {
            node_type: NodeType::PARAGRAPH,
            depth: depth + 1,
            parent_id: current,
            children: vec![],
            data: 0,
        });

        self.node_current = new_node;
        self.text_to_newline();

        self.node_current = current;
        self.current().push(new_node);
        return true;
    }
    pub fn passage(&mut self, new_line_break: bool) -> bool {
        let mut flag = false;
        loop {
            if self.heading() {
                flag = true;
            } else if self.paragraph(true) {
                flag = true;
            } else if new_line_break && self.checkout() == '\n' {
                break;
            } else {
                break;
            }
        }
        return flag;
    }
    pub fn heading(&mut self) -> bool {
        let ch = self.checkout();
        if ch == '#' {
            self.text_index += 1;
            let mut heading = 1;
            loop {
                let ch2 = self.checkout();
                if ch2 == '#' {
                    self.text_index += 1;
                    heading += 1;
                } else {
                    break;
                }
            }
            let current = self.node_current;
            let depth = self.current().depth;
            let new_node = self.node_pool.push(Node {
                node_type: NodeType::HEADING,
                depth: depth + 1,
                parent_id: current,
                children: vec![],
                data: heading,
            });

            self.node_current = new_node;
            self.text_to_newline();

            self.node_current = current;
            self.current().push(new_node);
            return true;
        }

        return false;
    }
    pub fn build(&mut self) {
        self.passage(false);
    }

    pub fn output(&mut self, buf: &mut String) {
        let debug = false;
//        let debug = true;
        if debug {
            buf.push_str(format!("{:#?}", &self).as_str());
        } else {
            self.parse(buf, self.node_root);
        }
    }
    pub fn parse(&self, buf: &mut String, node_i: usize) {
        let node = self.node_pool.get(node_i);
        match node.node_type {
            NodeType::CHAR => { buf.push(node.data as u8 as char); }
            NodeType::HEADING => {
                buf.push_str(format!("<h{}>", node.data).as_str());
                for i in node.children.iter() {
                    self.parse(buf, *i);
                }
                buf.push_str(format!("</h{}>", node.data).as_str());
            }
            NodeType::PARAGRAPH => {
                buf.push_str("<p>");
                for i in node.children.iter() {
                    self.parse(buf, *i);
                }
                buf.push_str("</p>");
            }
            NodeType::BOLD => {
                buf.push_str("<b>");
                for i in node.children.iter() {
                    self.parse(buf, *i);
                }
                buf.push_str("</b>");
            }
            NodeType::ITALIC => {
                buf.push_str("<em>");
                for i in node.children.iter() {
                    self.parse(buf, *i);
                }
                buf.push_str("</em>");
            }
            NodeType::PASSAGE => {
                for i in node.children.iter() {
                    self.parse(buf, *i);
                }
            }
            _ => { unimplemented!("{:?}", node.node_type); }
        }
    }
}
