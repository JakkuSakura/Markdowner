use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum DType<'a> {
    STRING(String),
    INTEGER(i32),
    FLOAT(f64),
    CHAR(char),
    POINTER(&'a DType<'a>),
    ARRAY(Vec<DType<'a>>),
    MAPPING(HashMap<String, DType<'a>>),
    NONE,
}

impl<'a> DType<'a> {
    fn from_str(s: &str) -> DType<'a> {
        DType::STRING(s.to_string())
    }
    fn as_str(&self) -> Option<&String> {
        match self {
            DType::STRING(s) => Some(s),
            _ => None
        }
    }
    fn from_char(ch: char) -> DType<'a> {
        DType::CHAR(ch)
    }
    fn as_char(&self) -> Option<char> {
        match self {
            DType::CHAR(c) => Some(*c),
            _ => None
        }
    }
    fn from_int(i: i32) -> DType<'a> {
        DType::INTEGER(i)
    }
    fn as_int(&self) -> Option<i32> {
        match self {
            DType::INTEGER(i) => Some(*i),
            _ => None
        }
    }
    fn from_float(f: f64) -> DType<'a> {
        DType::FLOAT(f)
    }

    fn as_float(&self) -> Option<f64> {
        match self {
            DType::FLOAT(f) => Some(*f),
            _ => None
        }
    }
    fn from_pointer(p: &'a DType) -> DType<'a> {
        DType::POINTER(p)
    }

    fn as_pointer(&self) -> Option<&'a DType> {
        match self {
            DType::POINTER(p) => Some(p),
            _ => None
        }
    }

    fn from_array(a: Vec<DType<'a>>) -> DType<'a> {
        DType::ARRAY(a)
    }

    fn as_array(&self) -> Option<&Vec<DType>> {
        match self {
            DType::ARRAY(a) => Some(a),
            _ => None
        }
    }
    fn new_map() -> DType<'a> {
        DType::MAPPING(Default::default())
    }
    fn from_map(a: HashMap<String, DType<'a>>) -> DType<'a> {
        DType::MAPPING(a)
    }

    fn as_map(&self) -> Option<&Vec<DType>> {
        match self {
            DType::ARRAY(a) => Some(a),
            _ => None
        }
    }
    fn as_map_mut(&mut self) -> Option<&mut Vec<DType<'a>>> {
        match self {
            DType::ARRAY(a) => Some(a),
            _ => None
        }
    }

    pub fn get(&self, key: &str) -> &DType<'a> {
        match self {
            DType::MAPPING(m) => {
                match m.get(key) {
                    Some(x) => x,
                    None => &DType::NONE
                }
            }
            _ => &DType::NONE
        }
    }

    pub fn set(&mut self, key: &str, val: DType<'a>) {
        match self {
            DType::MAPPING(m) => {
                m.insert(key.to_string(), val);
            }
            _ => panic!("Can not insert key-value pair into non mapping type")
        }
    }
}

#[derive(Debug)]
struct Pattern {
    name: String,
    arg: String,
    rep_least: i32,
    rep_most: i32,
}

impl Pattern {
    pub fn new() -> Pattern {
        Pattern {
            name: "undefined".to_string(),
            arg: "".to_string(),
            rep_least: 0,
            rep_most: 0,
        }
    }
    pub fn from_str(s: &str) -> Pattern {
        let mut cm = Pattern::new();


        let larg = s.find('(').unwrap();
        let rarg = s.find(')').unwrap();
        cm.name = s[0..larg].to_string();
        cm.arg = s[larg+1..rarg].to_string();

        let bracket = s.find('{').unwrap();
        let spt: Vec<&str> = s[bracket + 1..s.len() - 1].split(",").collect();
        cm.rep_least = i32::from_str(spt[0]).unwrap();
        cm.rep_most = i32::from_str(spt[1]).unwrap();
        cm
    }
}

#[derive(Debug)]
struct Grammar {
    node_name: String,
    patterns: Vec<Pattern>,
    action: String,
    action_arg: String,
}

impl Grammar {
    pub fn new(node_name: &str, action: &str, action_arg: &str) -> Grammar {
        Grammar {
            node_name: node_name.to_string(),
            patterns: vec![],
            action: action.to_string(),
            action_arg: action_arg.to_string(),
        }
    }
}


#[derive(Debug)]
struct Rule {
    name: String,
    grammars: Vec<Grammar>,
}

impl Rule {
    pub fn new() -> Rule {
        Rule {
            name: "undefined".to_string(),
            grammars: vec![],
        }
    }
}

struct DomBuilder<'a> {
    raw_text: &'a str,
    // action_name, function (arg, matched node) -> new node
    builtin_actions: HashMap<String, fn(&str, Vec<DType<'a>>) -> DType<'a>>,

    // pattern_name, function (arg, text, pos) -> (node, new pos) if matched
    builtin_patterns: HashMap<String, fn(&str, &str, isize) -> Option<(DType<'a>, isize)>>,
    rules: Vec<Rule>,
}

impl<'a> DomBuilder<'a> {
    pub fn new() -> DomBuilder<'a> {
        DomBuilder {
            raw_text: "",
            builtin_actions: Default::default(),
            builtin_patterns: Default::default(),
            rules: vec![],
        }
    }


    fn find_rule(&self, name: &str) -> Option<&Rule> {
        for e in &self.rules {
            if &e.name == name {
                return Some(e);
            }
        }
        return None;
    }
    fn match_pattern(&self, pt: &Pattern, token_pos: isize) -> Option<(DType<'a>, isize)> {
        match self.builtin_patterns.get(&pt.name) {
            Some(f) => { f(&pt.arg, self.raw_text, token_pos) }
            None => {
                match self.find_rule(&pt.name) {
                    Some(rule) => {
                        for e in &rule.grammars {
                            let result = self.match_grammar(e, token_pos);
                            if result != None {
                                return result;
                            }
                        }
                        None
                    }
                    None => {
                        panic!("Did not define built-in combination or rule: {}", pt.name);
                    }
                }
            }
        }
    }

    fn match_grammar(&self, grammar: &Grammar, token_pos: isize) -> Option<(DType<'a>, isize)> {
        let mut pos = token_pos;
        let mut matched: Vec<DType<'a>> = vec![];

        for e in &grammar.patterns {
            let mut rep = 0;
            let mut new_new_pos = pos;
            // what the hell?

            while rep < (e.rep_most as u32) {
                match self.match_pattern(&e, new_new_pos) {
                    Some((matched_pt, new_pos)) => {
                        rep += 1;
                        new_new_pos = new_pos;
                        matched.push(matched_pt);
                    }
                    None => { break; }
                }
            }
            if rep < e.rep_least as u32 || rep > e.rep_most as u32 {
                return Option::None;
            }
            pos = new_new_pos;
        }

        let action = self.builtin_actions.get(&grammar.action).unwrap();
        let new_node = action(&grammar.action_arg, matched);
        return Option::Some((new_node, pos));
    }

    fn process_grammar(&self, rule: &mut Rule, line: &str) {
        let args: Vec<&str> = line.split(" ").collect();
        let action_spt_line = args[1].find('(').unwrap();

        let mut gr = Grammar::new(&rule.name,
                                  &args[1][0..action_spt_line],
                                  &args[1][action_spt_line + 1..args[1].len() - 1]);
        for i in 2..args.len() {
            let par = args[i];
            let comb = Pattern::from_str(&par.to_string());
            gr.patterns.push(comb);
        }
        rule.grammars.push(gr);
    }

    fn read_rules(&mut self, filename: &str) {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let mut rule = Rule::new();
        for (index, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            let spt: Vec<&str> = line.split(char::is_whitespace).collect();
            match spt[0] {
                "#note" => { /* ignored*/ }
                "#def" => { rule.name = spt[1].to_string(); }
                "#grammar" => { self.process_grammar(&mut rule, &line); }
                "#end_def" => {
                    self.rules.push(rule);
                    rule = Rule::new();
                }
                "" => {}
                _ => { panic!("Can't resolve: {}", line); }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{DType, DomBuilder};

    #[test]
    fn it_works() {
        let mut node = DType::new_map();
        node.set("fuck", DType::INTEGER(233));
        assert_eq!(*node.get("fuck"), DType::INTEGER(233));
        assert_ne!(*node.get("fuck"), DType::INTEGER(2334));
        assert_eq!(*node.get("Fuck"), DType::NONE);
        node.set("fuck", DType::INTEGER(2334));
        assert_eq!(*node.get("fuck"), DType::INTEGER(2334));
    }

    fn is_char<'a>(arg: &str, text: &str, pos: isize) -> Option<(DType<'a>, isize)> {
        if pos < text.len() as isize {
            let ch = text.chars().nth(pos as usize).unwrap();
            if arg != "" || arg != "any" {
                let mut s = String::new();
                s.push('\'');
                s.push(ch);
                s.push('\'');
                if &s != arg {
                    return None;
                }
            }
            let mut node = DType::from_char(ch);
            return Some((node, pos + 1));
        }
        return None;
    }

    fn add_list<'a>(arg: &str, v: Vec<DType<'a>>) -> DType<'a> {
        let mut node = DType::new_map();
        node.set("name", DType::from_str(arg));
        node.set("matched", DType::from_array(v));
        return node;
    }

    #[test]
    fn dom_builder() {
        let mut dom_builder = DomBuilder::new();
        dom_builder.read_rules("rule.txt");
        dom_builder.raw_text = "hello";
        dom_builder.builtin_patterns.insert("char".to_string(), is_char);
        dom_builder.builtin_actions.insert("add_list".to_string(), add_list);
        let grammar = dom_builder.find_rule("text").unwrap().grammars.get(0).unwrap();
        let result = dom_builder.match_grammar(grammar, 0);
        assert_ne!(result, None);
        let (root, pos) = result.unwrap();
        println!("{:#?}", root);
    }
}
