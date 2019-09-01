trait Node {
    //    fn get_node_type(&self) -> &str;
    fn write_to_buf(&self, buf: &mut Vec<u8>);
}

trait VecU8Ex {
    fn new() -> Vec<u8>;
    fn from_num(i: i32) -> Vec<u8>;
    fn push_vec(&mut self, s: &Vec<u8>);
    fn push_str(&mut self, s: &str);
    fn push_char(&mut self, ch: char);
}

impl VecU8Ex for Vec<u8> {
    fn new() -> Vec<u8> {
        vec![] as Vec<u8>
    }
    fn from_num(i: i32) -> Vec<u8> {
        let mut buf = Vec::<u8>::new();
        buf.push_str(&format!("{}", i));
        return buf;
    }
    fn push_vec(&mut self, s: &Vec<u8>) {
        for e in s {
            self.push(*e);
        }
    }
    fn push_str(&mut self, s: &str) {
        for e in s.as_bytes() {
            self.push(*e);
        }
    }

    /// warning: this will only save low 8-bit of a char
    fn push_char(&mut self, ch: char) {
        self.push(ch as u8)
    }
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
    language: Vec<u8>,
    text: Vec<u8>,
}

struct Text {
    nodes: Vec<Box<dyn Node>>
}

struct Bold {
    text: Box<dyn Node>
}

struct MathDisplay {
    formula: Vec<u8>
}

struct Italic {
    text: Box<dyn Node>
}

struct Deleted {
    text: Box<dyn Node>
}

struct CodeInline {
    code: Vec<u8>
}

struct MathInline {
    formula: Vec<u8>
}


impl Node for Passage {
    fn write_to_buf(&self, buf: &mut Vec<u8>) {
        for x in &self.paragraphs {
            x.write_to_buf(buf);
        }
    }
}

impl Node for PlainParagraph {
    fn write_to_buf(&self, buf: &mut Vec<u8>) {
        buf.push_str("<p>");
        self.text.write_to_buf(buf);
        buf.push_str("</p>");
    }
}


impl Node for Heading {
    fn write_to_buf(&self, buf: &mut Vec<u8>) {
        buf.push_str(&format!("<h{}>", self.heading_level));
        self.text.write_to_buf(buf);
        buf.push_str(&format!("</h{}>", self.heading_level));
    }
}


impl Node for OrderedList {
    fn write_to_buf(&self, buf: &mut Vec<u8>) {
        unimplemented!()
    }
}


impl Node for UnorderedList {
    fn write_to_buf(&self, buf: &mut Vec<u8>) {
        unimplemented!()
    }
}


impl Node for Quote {
    fn write_to_buf(&self, buf: &mut Vec<u8>) {
        unimplemented!()
    }
}


impl Node for CodeBlock {
    fn write_to_buf(&self, buf: &mut Vec<u8>) {
        buf.push_str("<pre class=\"lang");
        if self.language.len() > 0 {
            buf.push_str("-");
            buf.push_vec(&self.language);
            buf.push_str("\"");
        }
        buf.push_vec(&self.text);
        buf.push_str("</pre>");
    }
}


impl Node for MathDisplay {
    fn write_to_buf(&self, buf: &mut Vec<u8>) {
        buf.push_str("<pre class=\"lang-math-display\">");
        buf.push_vec(&self.formula);
        buf.push_str("</pre>");
    }
}

impl Node for Text {
    fn write_to_buf(&self, buf: &mut Vec<u8>) {
        for x in &self.nodes {
            x.write_to_buf(buf);
        }
    }
}

impl Node for Bold {
    fn write_to_buf(&self, buf: &mut Vec<u8>) {
        buf.push_str("<b>");
        self.text.write_to_buf(buf);
        buf.push_str("</b>");
    }
}


impl Node for Italic {
    fn write_to_buf(&self, buf: &mut Vec<u8>) {
        buf.push_str("<i>");
        self.text.write_to_buf(buf);
        buf.push_str("</i>");
    }
}


impl Node for Deleted {
    fn write_to_buf(&self, buf: &mut Vec<u8>) {
        buf.push_str("<del>");
        self.text.write_to_buf(buf);
        buf.push_str("</del>");
    }
}


impl Node for CodeInline {
    fn write_to_buf(&self, buf: &mut Vec<u8>) {
        buf.push_str("<code>");
        buf.push_vec(&self.code);
        buf.push_str("</code>");
    }
}


impl Node for MathInline {
    fn write_to_buf(&self, buf: &mut Vec<u8>) {
        buf.push_str("<div class=\"lang-math-inline\">");
        buf.push_vec(&self.formula);
        buf.push_str("</div>");
    }
}

impl Node for u8 {
    fn write_to_buf(&self, buf: &mut Vec<u8>) {
        buf.push(*self);
    }
}

pub struct Parser {
    pub raw_text: Vec<u8>,
}

/// every function that returns Option<(Node, usize)>
/// returns a fine Node and the next possible position if succeeds
impl Parser {
    fn is(&self, pos: usize, s: &str) -> bool {
        if s == "" { return false; }
        if pos + s.len() > self.raw_text.len() {
            return false;
        }
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
    fn get_word(&self, pos: usize) -> Option<(Vec<u8>, usize)> {
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
        return Some((num, pos));
    }
    fn get_char(&self, pos: usize) -> Option<(u8, usize)> {
        if pos >= self.raw_text.len() {
            return None;
        }
        return Some((self.raw_text[pos], pos + 1));
    }
    fn eat(&self, pos: usize, s: &str, count: i32) -> usize {
        if s != "" && self.count(pos, s) < count {
            panic!("Cannot eat {} for {} time(s)", s, count);
        }
        return pos + s.len() * count as usize;
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
    pub fn parse(&self) -> Vec<u8> {
        match self.passage(0) {
            Some((p, pos)) => {
                let mut buf = Vec::<u8>::new();
                p.write_to_buf(&mut buf);
                return buf;
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
    fn boxing<T: Node + 'static>(x: Option<(T, usize)>) -> Option<(Box<dyn Node>, usize)> {
        x.map(|(y, z)| (Box::new(y) as _, z))
    }

    fn paragraph(&self, pos: usize) -> Option<(Box<dyn Node>, usize)> {
        let heading = self.heading(pos);
        if heading.is_some() { return Parser::boxing(heading); }
        let ordered_list = self.ordered_list(pos);
        if ordered_list.is_some() { return Parser::boxing(ordered_list); }

        let unordered_list = self.unordered_list(pos);
        if unordered_list.is_some() { return Parser::boxing(unordered_list); }

        let quote = self.quote(pos);
        if quote.is_some() { return Parser::boxing(quote); }

        let code_block = self.code_block(pos);
        if code_block.is_some() { return Parser::boxing(code_block); }

        let math_display = self.math_display(pos);
        if math_display.is_some() { return Parser::boxing(math_display); }


        let plain = self.plain(pos);
        if plain.is_some() { return Parser::boxing(plain); }

        return None;
    }
    fn plain(&self, pos: usize) -> Option<(PlainParagraph, usize)> {
        match self.text(pos, false, "") {
            Some((x, y)) => {
                Some((PlainParagraph { text: Box::new((x)) }, y))
            }
            None => None
        }
    }
    fn heading(&self, pos: usize) -> Option<(Heading, usize)> {
        let number = self.count(pos, "#");
        if number < 1 { return None; }
        let pos = self.eat(pos, "#", number);
        let (text, new_pos) = self.text(pos, false, "").unwrap();

        return Some((Heading { heading_level: number, text: Box::new(text) }, new_pos));
    }
    fn ordered_list(&self, pos: usize) -> Option<(OrderedList, usize)> {
        match self.get_number(pos) {
            Some((x, pos1)) => {
                if !self.is(pos1, ". ") {
                    return None;
                }
                let pos2 = self.eat(pos1, ". ", 1);
                let (text, pos3) = self.text(pos2, false, "").unwrap();
                return Some((OrderedList { text: Box::new(text) }, pos3));
            }
            None => None
        }
    }
    fn unordered_list(&self, pos: usize) -> Option<(UnorderedList, usize)> {
        match self.is(pos, "- ") {
            true => {
                let pos2 = self.eat(pos, "- ", 1);
                let (text, pos3) = self.text(pos2, false, "").unwrap();
                return Some((UnorderedList { text: Box::new(text) }, pos3));
            }
            false => None
        }
    }
    fn quote(&self, pos: usize) -> Option<(Quote, usize)> {
        let number = self.count(pos, ">");
        if number < 1 { return None; }
        let pos = self.eat(pos, ">", number);
        let (text, new_pos) = self.text(pos, false, "").unwrap();

        return Some((Quote { quote_level: number, text: Box::new(text) }, new_pos));
    }
    fn code_block(&self, pos: usize) -> Option<(CodeBlock, usize)> {
        if self.is(pos, "```") {
            let mut pos2 = pos + 3;
            let lang;
            match self.get_word(pos2) {
                Some((s, pos3)) => {
                    lang = s;
                    pos2 = pos3;
                }
                None => lang = vec![]
            }
            let mut text = Vec::<u8>::new();
            while pos2 < self.raw_text.len() {
                if self.is(pos2, "```") {
                    pos2 += 3;
                    break;
                }
                text.push(self.raw_text[pos2]);
                pos2 += 1;
            }
            return Some((CodeBlock { language: lang, text }, pos2));
        }
        return None;
    }
    fn math_display(&self, pos: usize) -> Option<(MathDisplay, usize)> {
        if self.is(pos, "$$") {
            let mut pos2 = pos + 2;
            let mut text = Vec::<u8>::new();
            while pos2 < self.raw_text.len() {
                if self.is(pos2, "$$") {
                    pos2 += 2;
                    break;
                }
                text.push(self.raw_text[pos2]);
                pos2 += 1;
            }
            return Some((MathDisplay { formula: text }, pos2));
        }
        return None;
    }
    fn text(&self, pos: usize, linebreak: bool, stop_at: &str) -> Option<(Text, usize)> {
        let mut pos = pos;
        let mut text = Text { nodes: vec![] };
        let mut cond = true;
        while cond {
            cond = false;
            if self.is(pos, stop_at) {
                pos += stop_at.len();
                break;
            }
            match self.bold(pos) {
                Some((b, p)) => {
                    text.nodes.push(Box::new(b));
                    pos = p;
                    cond = true;
                }
                None => {}
            }
            match self.italic(pos) {
                Some((i, p)) => {
                    text.nodes.push(Box::new(i));
                    pos = p;
                    cond = true;
                }
                None => {}
            }
            match self.deleted(pos) {
                Some((d, p)) => {
                    text.nodes.push(Box::new(d));
                    pos = p;
                    cond = true;
                }
                None => {}
            }
            match self.code_inline(pos) {
                Some((c, p)) => {
                    text.nodes.push(Box::new(c));
                    pos = p;
                    cond = true;
                }
                None => {}
            }
            match self.math_inline(pos) {
                Some((m, p)) => {
                    text.nodes.push(Box::new(m));
                    pos = p;
                    cond = true;
                }
                None => {}
            }
            match self.get_char(pos) {
                Some((c, p)) => {
                    if !linebreak && c == '\n' as u8 { break; }

                    text.nodes.push(Box::new(c));
                    pos = p;
                    cond = true;
                }
                None => {}
            }
        }

        return Some((text, pos));
    }
    fn bold(&self, pos: usize) -> Option<(Bold, usize)> {
        if self.is(pos, "**") {
            let pos = pos + 2;
            match self.text(pos, false, "**") {
                Some((x, y)) => Some((Bold { text: Box::new(x) }, y)),
                None => None
            };
        }
        None
    }
    fn italic(&self, pos: usize) -> Option<(Italic, usize)> {
        if self.is(pos, "*") {
            let pos = pos + 1;
            match self.text(pos, false, "*") {
                Some((x, y)) => Some((Italic { text: Box::new(x) }, y)),
                None => None
            };
        }
        None
    }
    fn deleted(&self, pos: usize) -> Option<(Deleted, usize)> {
        if self.is(pos, "~~") {
            let pos = pos + 2;
            match self.text(pos, false, "~~") {
                Some((x, y)) => Some((Deleted { text: Box::new(x) }, y)),
                None => None
            };
        }
        None
    }
    fn code_inline(&self, pos: usize) -> Option<(CodeInline, usize)> {
        if self.is(pos, "`") {
            let mut pos2 = pos + 1;
            let mut text = Vec::<u8>::new();
            while pos2 < self.raw_text.len() {
                if self.is(pos2, "\n") || self.is(pos2, "`") {
                    pos2 += 1;
                    break;
                }
                text.push(self.raw_text[pos2]);
                pos2 += 1;
            }
            return Some((CodeInline { code: text }, pos2));
        }
        return None;
    }
    fn math_inline(&self, pos: usize) -> Option<(MathInline, usize)> {
        if self.is(pos, "$") {
            let mut pos2 = pos + 1;
            let mut text = Vec::<u8>::new();
            while pos2 < self.raw_text.len() {
                if self.is(pos2, "\n") || self.is(pos2, "$") {
                    pos2 += 1;
                    break;
                }
                text.push(self.raw_text[pos2]);
                pos2 += 1;
            }
            return Some((MathInline { formula: text }, pos2));
        }
        return None;
    }
}