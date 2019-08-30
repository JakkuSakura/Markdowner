use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use std::str::FromStr;


#[derive(Debug, PartialEq)]
pub enum DynamicType<'a> {
    STRING(&'a str),
    INTEGER(i32),
    FLOAT(f64),
    POINTER(&'a DynamicType<'a>),
    ARRAY(Vec<DynamicType<'a>>),
    NONE,
}

impl<'a> DynamicType<'a> {
    fn as_str(&self) -> Option<&str> {
        match self {
            DynamicType::STRING(s) => Some(s),
            _ => None
        }
    }
    fn as_int(&self) -> Option<i32> {
        match self {
            DynamicType::INTEGER(i) => Some(*i),
            _ => None
        }
    }
    fn as_float(&self) -> Option<f64> {
        match self {
            DynamicType::FLOAT(f) => Some(*f),
            _ => None
        }
    }
    fn as_pointer(&self) -> Option<&'a DynamicType> {
        match self {
            DynamicType::POINTER(p) => Some(p),
            _ => None
        }
    }
    fn as_array(&self) -> Option<&Vec<DynamicType>> {
        match self {
            DynamicType::ARRAY(a) => Some(a),
            _ => None
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum DynamicNode<'a> {
    NODE { name: String, properties: HashMap<&'a str, DynamicType<'a>> },
    NONE,
}

impl<'a> DynamicNode<'a> {
    pub fn new(name: String) -> DynamicNode<'a> {
        DynamicNode::NODE { name, properties: HashMap::new() }
    }
    pub fn get(&self, key: &str) -> &DynamicType<'a> {
        match self {
            DynamicNode::NODE { name, properties } => {
                match properties.get(key) {
                    Some(x) => x,
                    None => &DynamicType::NONE
                }
            }
            _ => panic!("This DynamicNode is NONE")
        }
    }


    pub fn set(&mut self, key: &'a str, val: DynamicType<'a>) {
        match self {
            DynamicNode::NODE { name, properties } => {
                properties.insert(key, val);
            }
            _ => panic!("This DynamicNode is NONE. Cannot set kay-value mapping")
        }
    }
}

#[derive(Debug)]
struct Action {
    name: String,
    args: Vec<String>,
}

impl Action {
    pub fn from_str(s: &str) -> Action {
        let parenthesis_index = s.find("(").unwrap();
        let args = s[parenthesis_index + 1..s.len() - 1].to_string();
        let spt: Vec<&str> = args.split(",").collect();
        let mut vec: Vec<String> = vec![];
        if spt[0] != "" {
            for s in spt {
                vec.push(s.to_string());
            }
        }
        let action = Action { name: s[0..parenthesis_index].to_string(), args: vec };
        action
    }
}

#[derive(Debug)]
struct Combination {
    name: String,
    rep_least: i32,
    rep_most: i32,
    action: Action,
}

impl Combination {
    pub fn new() -> Combination {
        Combination {
            name: "undefined".to_string(),
            rep_least: 0,
            rep_most: 0,
            action: Action { name: "".to_string(), args: vec![] },
        }
    }
    pub fn from_str(s: &str) -> Combination {
        let mut cm = Combination::new();

        let bracket = s.find('{').unwrap();
        cm.name = s[0..bracket].to_string();
        let spt: Vec<&str> = s[bracket + 1..s.len() - 1].split(",").collect();
        cm.rep_least = i32::from_str(spt[0]).unwrap();
        cm.rep_most = i32::from_str(spt[1]).unwrap();
// todo multi args support
        cm.action = Action::from_str(&spt[2].to_string());
        cm
    }
}

#[derive(Debug)]
struct Grammar {
    node_name: String,
    combinations: Vec<Combination>,
}

impl Grammar {
    pub fn new(node_name: String) -> Grammar {
        Grammar { node_name, combinations: vec![] }
    }
}


#[derive(Debug)]
struct Rule {
    name: String,
    mem: Vec<(String, String)>,
    grammars: Vec<Grammar>,
}

impl Rule {
    pub fn new() -> Rule {
        Rule {
            name: "undefined".to_string(),
            mem: vec![],
            grammars: vec![],
        }
    }
}

struct DomBuilder<'a> {
    raw_text: &'a str,
    builtin_actions: HashMap<String, fn(&Action, RefMut<DynamicNode<'a>>, &DynamicNode<'a>)>,
    // action_name, function (action, root_node, current_node)
    builtin_combinations: HashMap<String, fn(&str, pos) -> (DynamicNode<'a>, isize)>,
    rules: Vec<Rule>,
}

impl<'a> DomBuilder<'a> {
    pub fn new() -> DomBuilder<'a> {
        DomBuilder {
            raw_text: "",
            builtin_actions: Default::default(),
            builtin_combinations: Default::default(),
            rules: vec![],
        }
    }
    fn do_action(&self, action: &Action, node: RefMut<DynamicNode<'a>>, matched: &DynamicNode<'a>) {
        let func = self.builtin_actions.get(action.name.as_str()).unwrap();
        func(action, node, matched);
    }

    fn match_combination(&self, comb: &Combination, token_pos: isize) -> (DynamicNode<'a>, isize) {
        match self.builtin_combinations.get(&comb.name) {
            Some(f) => { f(self.raw_text, token_pos) }
            None => {
                match self.rules.get(&comb.name) {
                    Some(rule) => {
                        unimplemented!();

                    },
                    None => {
                        panic!("Did not define built-in combination or rule: {}", comb.name);
                    }
                }
            }
        }
    }

    fn match_grammar(&self, grammar: &Grammar, token_pos: isize) -> (DynamicNode<'a>, isize) {
        let mut pos = token_pos;
        let node = RefCell::new(DynamicNode::new(grammar.node_name.to_string()));
        for e in &grammar.combinations {
            let mut rep = 0;
            let mut new_new_pos = pos;
            // what the hell?

            while rep < e.rep_most {
                let (matched_comb, new_pos) = self.match_combination(&e, new_new_pos);
                if matched_comb == DynamicNode::NONE {
                    break;
                }
                rep += 1;
                new_new_pos = new_pos;
                self.do_action(&e.action, node.borrow_mut(), &matched_comb);
            }

            if rep < e.rep_least || rep > e.rep_most {
                return (DynamicNode::NONE, new_new_pos);
            }
            pos = new_new_pos;
        }
        return (node.into_inner(), pos);
    }
}

fn process_grammar(rule: &mut Rule, line: &str) {
    let args: Vec<&str> = line.split(" ").collect();
    let mut gr = Grammar::new(rule.name.clone());
    for i in 1..args.len() {
        let par = args[i];
        let comb = Combination::from_str(&par.to_string());
        gr.combinations.push(comb);
    }
    rule.grammars.push(gr);
}

fn read_rules(filename: &str) -> Vec<Rule> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut rules = vec![];
    let mut rule = Rule::new();
    ;
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let spt: Vec<&str> = line.split(char::is_whitespace).collect();
        match spt[0] {
            "#note" => { /* ignored*/ }
            "#def" => { rule.name = spt[1].to_string(); }
            "#mem" => { rule.mem.push((spt[1].to_string(), spt[2].to_string())) }
            "#grammar" => { process_grammar(&mut rule, &line); }
            "#end_def" => {
                rules.push(rule);
                rule = Rule::new();
            }
            "" => {}
            _ => { panic!("Can't resolve: {}", line); }
        }
    }
    return rules;
}

#[cfg(test)]
mod tests {
    use crate::{DynamicNode, DynamicType, DomBuilder};

    #[test]
    fn it_works() {
        let mut node = DynamicNode::new("hello".to_string());
        node.set("fuck", DynamicType::INTEGER(233));
        assert_eq!(*node.get("fuck"), DynamicType::INTEGER(233));
        assert_ne!(*node.get("fuck"), DynamicType::INTEGER(2334));
        assert_eq!(*node.get("Fuck"), DynamicType::NONE);
        node.set("fuck", DynamicType::INTEGER(2334));
        assert_eq!(*node.get("fuck"), DynamicType::INTEGER(2334));
    }

    #[test]
    fn read_rules() {
        let rules = super::read_rules("rule.txt");
//        panic!("{:#?}", rules);
    }

    #[test]
    fn dom_builder() {
        let rules = super::read_rules("rule.txt");
        let mut dom_builder = DomBuilder::new();
        dom_builder.rules = rules;
    }
}
