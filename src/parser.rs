trait Node {
    //    fn get_node_type(&self) -> &str;
    fn get_string(&self) -> String;
}


struct Passage {
    paragraphs: Vec<Box<dyn Node>>
}


struct PlainParagraph {
    text: Box<dyn Node>
}


struct Heading {
    heading_level: i32,
    text: Box<dyn Node>,
}

struct OrderedList {
    text: Box<dyn Node>
}

struct UnorderedList {
    text: Box<dyn Node>
}

struct Quote {
    quote_level: i32,
    text: Box<dyn Node>,
}

struct CodeBlock {
    language: String,
    text: String,
}

struct Bold {
    text: Box<dyn Node>
}

struct MathDisplay {
    formula: String
}

struct Italic {
    text: Box<dyn Node>
}

struct Deleted {
    text: Box<dyn Node>
}

struct CodeInline {
    code: String
}

struct MathInline {
    formula: String
}

impl Node for Passage {
    fn get_string(&self) -> String {
        let mut buf = String::new();
        for x in &self.paragraphs {
            buf.push_str(&x.get_string());
        }
        return buf;
    }
}

impl Node for PlainParagraph {
    fn get_string(&self) -> String {
        format!("<p>{}</p>", self.text.get_string())
    }
}


impl Node for Heading {
    fn get_string(&self) -> String {
        format!("<h{}>{}</h{}>", self.heading_level, self.text.get_string(), self.heading_level)
    }
}


impl Node for OrderedList {
    fn get_string(&self) -> String {
        unimplemented!()
    }
}


impl Node for UnorderedList {
    fn get_string(&self) -> String {
        unimplemented!()
    }
}


impl Node for Quote {
    fn get_string(&self) -> String {
        unimplemented!()
    }
}


impl Node for CodeBlock {
    fn get_string(&self) -> String {
        format!("<pre class=\"lang-{}\">{}</pre>", self.language, self.text)
    }
}


impl Node for MathDisplay {
    fn get_string(&self) -> String {
        format!("<code class=\"lang-math-display\">{}</code>", self.formula)
    }
}


impl Node for Bold {
    fn get_string(&self) -> String {
        format!("<b>{}</b>", self.text.get_string())
    }
}


impl Node for Italic {
    fn get_string(&self) -> String {
        format!("<i>{}</i>", self.text.get_string())
    }
}


impl Node for Deleted {
    fn get_string(&self) -> String {
        format!("<del>{}</del>", self.text.get_string())
    }
}


impl Node for CodeInline {
    fn get_string(&self) -> String {
        format!("<code>{}</code>", self.code)
    }
}


impl Node for MathInline {
    fn get_string(&self) -> String {
        format!("<div class=\"lang-math-inline\">{}</div>", self.formula)
    }
}

impl Node for u8 {
    fn get_string(&self) -> String {
        format!("{}", self)
    }
}

struct Parser {
    raw_text: Vec<u8>,
}

/// every function that returns Option<(Node, usize)>
/// returns a fine Node and the next possible position if succeeds
impl Parser {
    fn is(&self, pos: usize, s: &str) -> bool {
        let xx = s.as_bytes();
        for i in 0..xx.len() {
            if self.raw_text[pos + i] != xx[i] {
                return false;
            }
        }
        return true;
    }
    fn get_number(&self, pos: usize) -> Option<(i32, usize)> {
        if !self.raw_text[pos].is_ascii_digit() {
            return None;
        }
        let mut num = 0i32;
        let mut pos = pos;
        while pos < self.raw_text.len() {
            if !self.raw_text[pos].is_ascii_digit() {
                break;
            }
            num = num * 10 + self.raw_text[pos] as i32 - 48;
            pos += 1;
        }
        return Some((num, pos));
    }
    fn get_word(&self, pos: usize) -> Option<(String, usize)> {
        if !self.raw_text[pos].is_ascii_alphanumeric() {
            return None;
        }
        let mut num: Vec<u8> = vec![];
        let mut pos = pos;
        while pos < self.raw_text.len() {
            if !self.raw_text[pos].is_ascii_alphanumeric() {
                break;
            }
            num.push(self.raw_text[pos]);
            pos += 1;
        }
        return Some((String::from_utf8(num).unwrap(), pos));
    }
    fn get_char(&self, pos: usize) -> Option<(u8, usize)> {
        if pos >= self.raw_text.len() {
            return None;
        }
        return Some((self.raw_text[pos], pos + 1));
    }
    fn count(&self, pos: usize, s: &str) -> i32 {
        let mut cnt = 0;
        let mut p = pos;
        while p + s.len() <= self.raw_text.len() {
            if self.is(p, s) {
                p += s.len();
                cnt += 1;
            } else { break; }
        }
        return cnt;
    }
    fn parse(&self) -> String {
        match self.passage(0) {
            Some((p, pos)) => {
                p.get_string()
            }
            None => panic!("WTF")
        }
    }
    fn passage(&self, pos: usize) -> Option<(Passage, usize)> {
        let mut pos = pos;
        let mut psg = Passage { paragraphs: vec![] };
        while pos < self.raw_text.len() {
            match self.paragraph(pos) {
                Some((x, p)) => {
                    psg.paragraphs.push(x);
                    pos = p;
                }
                None => panic!("WTF2")
            }
        }
        return Some((psg, pos));
    }
    fn boxing<T: Node>(x: Option<(T, usize)>) -> Option<(Box<dyn Node>, usize)> {
        x.map(|(y, z)| (Box::new(y) as _, z))
    }
    
    fn paragraph(&self, pos: usize) -> Option<(Box<dyn Node>, usize)> {
        let heading = self.heading(pos);
        if heading.is_some() { return heading; }
        let ordered_list = self.ordered_list(pos);
        if ordered_list.is_some() { return ordered_list; }

        let unordered_list = self.unordered_list(pos);
        if unordered_list.is_some() { return unordered_list; }

        let quote = self.quote(pos);
        if quote.is_some() { return quote; }

        let code_block = self.code_block(pos);
        if code_block.is_some() { return code_block; }

        let math_display = self.math_display(pos);
        if math_display.is_some() { return math_display; }


        let plain = self.plain(pos);
        if plain.is_some() { return plain; }

        return None;
    }
    fn plain(&self, pos: usize) -> Option<(PlainParagraph, usize)> { unimplemented!() }
    fn heading(&self, pos: usize) -> Option<(Heading, usize)> { unimplemented!() }
    fn ordered_list(&self, pos: usize) -> Option<(OrderedList, usize)> { unimplemented!() }
    fn unordered_list(&self, pos: usize) -> Option<(UnorderedList, usize)> { unimplemented!() }
    fn quote(&self, pos: usize) -> Option<(Quote, usize)> { unimplemented!() }
    fn code_block(&self, pos: usize) -> Option<(CodeBlock, usize)> { unimplemented!() }
    fn math_display(&self, pos: usize) -> Option<(MathDisplay, usize)> { unimplemented!() }
    fn text(&self, pos: usize) -> Option<(Box<dyn Node>, usize)> { unimplemented!() }

    fn bold(&self, pos: usize) -> Option<(Bold, usize)> { unimplemented!() }
    fn italic(&self, pos: usize) -> Option<(Italic, usize)> { unimplemented!() }
    fn deleted(&self, pos: usize) -> Option<(Deleted, usize)> { unimplemented!() }
    fn code_inline(&self, pos: usize) -> Option<(CodeInline, usize)> { unimplemented!() }
    fn math_inline(&self, pos: usize) -> Option<(MathInline, usize)> { unimplemented!() }
}