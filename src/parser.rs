use std::any::{Any, TypeId};
use std::cmp::{min, max};

#[cfg(debug_assertions)]
extern crate termcolor;

use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

trait Node: Any {
    #[cfg(debug_assertions)]
    fn get_node_type(&self) -> &str;

    fn write_to_buf(&self, buf: &mut dyn Buf);

    /// return the possible count of inner bytes within this node
    fn len(&self) -> i32;
}

pub type InnerByte = u8;
pub type InnerBuffer = Vec<InnerByte>;

pub trait Buf {
    fn push_vec(&mut self, s: &InnerBuffer);
    fn push_str(&mut self, s: &str);
    fn push_char(&mut self, ch: char);
    fn push(&mut self, ch: InnerByte);
}


struct Passage {
    paragraphs: Vec<Box<dyn Node>>
}


struct PlainParagraph {
    text: Box<dyn Node>
}


struct Heading {
    rank: i32,
    text: Box<dyn Node>,
}

struct OrderedList {
    list: Vec<Box<dyn Node>>
}

struct UnorderedList {
    list: Vec<Box<dyn Node>>
}

struct Quote {
    list: Vec<Box<dyn Node>>,
}

struct CodeBlock {
    language: InnerBuffer,
    text: InnerBuffer,
}

struct Text {
    nodes: Vec<Box<dyn Node>>
}

struct Bold {
    text: Box<dyn Node>
}

struct MathDisplay {
    formula: InnerBuffer
}

struct Italic {
    text: Box<dyn Node>
}

struct Deleted {
    text: Box<dyn Node>
}

struct CodeInline {
    code: InnerBuffer
}

struct MathInline {
    formula: InnerBuffer
}


impl Node for Passage {
    #[cfg(debug_assertions)]
    fn get_node_type(&self) -> &str {
        "Passage"
    }

    fn write_to_buf(&self, buf: &mut dyn Buf) {
        self.paragraphs.iter().for_each(|x| x.write_to_buf(buf));
    }

    fn len(&self) -> i32 {
        let mut x = 0;
        for y in &self.paragraphs {
            x += y.len();
        }
        return x;
    }
}

impl Node for PlainParagraph {
    #[cfg(debug_assertions)]
    fn get_node_type(&self) -> &str {
        "PlainParagraph"
    }

    fn write_to_buf(&self, buf: &mut dyn Buf) {
        buf.push_str("<p>");
        self.text.write_to_buf(buf);
        buf.push_str("</p>");
    }

    fn len(&self) -> i32 {
        self.text.len()
    }
}


impl Node for Heading {
    #[cfg(debug_assertions)]
    fn get_node_type(&self) -> &str {
        "Heading"
    }

    fn write_to_buf(&self, buf: &mut dyn Buf) {
        buf.push_str(&format!("<h{}>", self.rank));
        self.text.write_to_buf(buf);
        buf.push_str(&format!("</h{}>", self.rank));
    }

    fn len(&self) -> i32 {
        self.text.len()
    }
}


impl Node for OrderedList {
    #[cfg(debug_assertions)]
    fn get_node_type(&self) -> &str {
        "OrderedList"
    }

    fn write_to_buf(&self, buf: &mut dyn Buf) {
        buf.push_str("<ol>");
        for x in &self.list {
            buf.push_str("<li>");
            x.write_to_buf(buf);
            buf.push_str("</li>");
        }
        buf.push_str("</ol>");
    }

    fn len(&self) -> i32 {
        let mut x = 0;
        self.list.iter().for_each(|y| x += y.len());
        return x;
    }
}


impl Node for UnorderedList {
    #[cfg(debug_assertions)]
    fn get_node_type(&self) -> &str {
        "UnorderedList"
    }

    fn write_to_buf(&self, buf: &mut dyn Buf) {
        buf.push_str("<ul>");
        for x in &self.list {
            buf.push_str("<li>");
            x.write_to_buf(buf);
            buf.push_str("</li>");
        }
        buf.push_str("</ul>");
    }

    fn len(&self) -> i32 {
        let mut x = 0;
        self.list.iter().for_each(|y| x += y.len());
        return x;
    }
}


impl Node for Quote {
    #[cfg(debug_assertions)]
    fn get_node_type(&self) -> &str {
        "Quote"
    }

    fn write_to_buf(&self, buf: &mut dyn Buf) {
        buf.push_str("<blockquote>");
        for x in &self.list {
            buf.push_str("<p>");
            x.write_to_buf(buf);
            buf.push_str("<p>");
        }
        buf.push_str("</blockquote>");
    }

    fn len(&self) -> i32 {
        let mut x = 0;
        self.list.iter().for_each(|y| x += y.len());
        return x;
    }
}


impl Node for CodeBlock {
    #[cfg(debug_assertions)]
    fn get_node_type(&self) -> &str {
        "CodeBlock"
    }

    fn write_to_buf(&self, buf: &mut dyn Buf) {
        buf.push_str("<pre class=\"lang");
        if self.language.len() > 0 {
            buf.push_str("-");
            buf.push_vec(&self.language);
            buf.push_str("\"");
        }
        buf.push_str(">");
        buf.push_vec(&self.text);
        buf.push_str("</pre>");
    }

    fn len(&self) -> i32 {
        (self.language.len() + self.text.len()) as i32
    }
}


impl Node for MathDisplay {
    #[cfg(debug_assertions)]
    fn get_node_type(&self) -> &str {
        "MathDisplay"
    }

    fn write_to_buf(&self, buf: &mut dyn Buf) {
        buf.push_str("<pre class=\"lang-math-display\">");
        buf.push_vec(&self.formula);
        buf.push_str("</pre>");
    }

    fn len(&self) -> i32 {
        self.formula.len() as i32
    }
}

impl Node for Text {
    #[cfg(debug_assertions)]
    fn get_node_type(&self) -> &str {
        "Text"
    }

    fn write_to_buf(&self, buf: &mut dyn Buf) {
        for x in &self.nodes {
            x.write_to_buf(buf);
        }
    }

    fn len(&self) -> i32 {
        let mut y = 0;
        self.nodes.iter().for_each(|x| y += x.len());
        return y;
    }
}

impl Node for Bold {
    #[cfg(debug_assertions)]
    fn get_node_type(&self) -> &str {
        "Bold"
    }

    fn write_to_buf(&self, buf: &mut dyn Buf) {
        buf.push_str("<b>");
        self.text.write_to_buf(buf);
        buf.push_str("</b>");
    }

    fn len(&self) -> i32 {
        self.text.len()
    }
}


impl Node for Italic {
    #[cfg(debug_assertions)]
    fn get_node_type(&self) -> &str {
        "Italic"
    }

    fn write_to_buf(&self, buf: &mut dyn Buf) {
        buf.push_str("<i>");
        self.text.write_to_buf(buf);
        buf.push_str("</i>");
    }

    fn len(&self) -> i32 {
        self.text.len()
    }
}


impl Node for Deleted {
    #[cfg(debug_assertions)]
    fn get_node_type(&self) -> &str {
        "Deleted"
    }

    fn write_to_buf(&self, buf: &mut dyn Buf) {
        buf.push_str("<del>");
        self.text.write_to_buf(buf);
        buf.push_str("</del>");
    }

    fn len(&self) -> i32 {
        self.text.len()
    }
}


impl Node for CodeInline {
    #[cfg(debug_assertions)]
    fn get_node_type(&self) -> &str {
        "CodeInline"
    }

    fn write_to_buf(&self, buf: &mut dyn Buf) {
        buf.push_str("<code>");
        buf.push_vec(&self.code);
        buf.push_str("</code>");
    }

    fn len(&self) -> i32 {
        self.code.len() as i32
    }
}


impl Node for MathInline {
    #[cfg(debug_assertions)]
    fn get_node_type(&self) -> &str {
        "MathInline"
    }

    fn write_to_buf(&self, buf: &mut dyn Buf) {
        buf.push_str("<div class=\"lang-math-inline\">");
        buf.push_vec(&self.formula);
        buf.push_str("</div>");
    }

    fn len(&self) -> i32 {
        self.formula.len() as i32
    }
}

impl Node for InnerByte {
    #[cfg(debug_assertions)]
    fn get_node_type(&self) -> &str {
        "InnerByte"
    }

    fn write_to_buf(&self, buf: &mut dyn Buf) {
        buf.push(*self);
    }

    fn len(&self) -> i32 {
        1
    }
}

pub struct Parser {
    pub raw_text: InnerBuffer,
}

fn encase<T: Node + 'static>(x: Option<(T, usize)>) -> Option<(Box<dyn Node>, usize)> {
    x.map(|(y, z)| (Box::new(y) as _, z))
}


#[cfg(debug_assertions)]
fn show_slice(text: &InnerBuffer, begin: usize, end: usize, highlight_begin: usize, hightlight_end: usize) {
    let begin = max(begin, 0);
    let end = min(end, text.len());


    println!("[{}..{})", begin, end);
    for i in begin..end {
        if i >= highlight_begin && i < hightlight_end {
            let mut stdout = StandardStream::stdout(ColorChoice::Always);
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
            write!(&mut stdout, "{}", text[i] as char).unwrap();
            stdout.reset();
        } else {
            print!("{}", text[i] as char);
        }
    }
    println!();
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
    //    fn get_number(&self, pos: usize) -> Option<(i32, usize)> {
//        if !self.raw_text[pos].is_ascii_digit() {
//            return None;
//        }
//        let mut num = 0i32;
//        let mut pos = pos;
//        while pos < self.raw_text.len() {
//            if !self.raw_text[pos].is_ascii_digit() {
//                break;
//            }
//            num = num * 10 + self.raw_text[pos] as i32 - 48;
//            pos += 1;
//        }
//        return Some((num, pos));
//    }
    fn get_word(&self, pos: usize) -> Option<(InnerBuffer, usize)> {
        if !self.raw_text[pos].is_ascii_alphanumeric() {
            return None;
        }
        let mut num: InnerBuffer = vec![];
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
    fn get_char(&self, pos: usize) -> Option<InnerByte> {
        if pos >= self.raw_text.len() {
            return None;
        }
        return Some(self.raw_text[pos]);
    }
    fn check_eat(&self, pos: usize, s: &str, count: i32) -> usize {
        if s != "" {
            if self.count(pos, s) < count {
                panic!("Cannot eat {} for {} time(s)", s, count);
            }
            return pos + s.len() * count as usize;
        } else {
            return pos + count as usize;
        }
    }
    fn eat(&self, pos: usize) -> usize { pos + 1 }
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
    pub fn parse(&self, buf: &mut dyn Buf) {
        match self.passage(0) {
            Some((mut p, _)) => {
                Parser::readjust(&mut p);
                p.write_to_buf(buf);
            }
            None => panic!("unexpected error: unknown reason")
        }
    }

    fn readjust(node: &mut dyn Node) {
        fn is<T0: ?Sized + Any, T: ?Sized + Any>(_s: &T) -> bool {
            TypeId::of::<T0>() == TypeId::of::<T>()
        }

        if is::<OrderedList, _>(node) {
            println!("Node is ordered list");
        }
    }
    fn passage(&self, pos: usize) -> Option<(Passage, usize)> {
        let mut pos = pos;
        let mut psg = Passage { paragraphs: vec![] };
        while pos < self.raw_text.len() {
            match self.paragraph(pos) {
                Some((x, p)) => {
//                    println!("Matched {} before {}", x.get_node_type(), p);
                    psg.paragraphs.push(x);
                    if pos == p {
                        panic!("Unexpected error: same location: {}", p);
                    }
                    pos = p;
                }
                None => panic!("Unexpected error: unknown reason")
            }
        }
        return Some((psg, pos));
    }

    fn paragraph(&self, pos: usize) -> Option<(Box<dyn Node>, usize)> {
        let heading = self.heading(pos);
        if heading.is_some() { return encase(heading); }
        let ordered_list = self.ordered_list(pos);
        if ordered_list.is_some() { return encase(ordered_list); }

        let unordered_list = self.unordered_list(pos);
        if unordered_list.is_some() { return encase(unordered_list); }

        let quote = self.quote(pos);
        if quote.is_some() { return encase(quote); }

        let code_block = self.code_block(pos);
        if code_block.is_some() { return encase(code_block); }

        let math_display = self.math_display(pos);
        if math_display.is_some() { return encase(math_display); }

        let plain_paragraph = self.plain_paragraph(pos);
        if plain_paragraph.is_some() { return encase(plain_paragraph); }

        return None;
    }
    fn plain_paragraph(&self, pos: usize) -> Option<(PlainParagraph, usize)> {
        match self.text(pos, false, "", true, true, true) {
            Some((x, y)) => {
//                if x.len() > 0 {
                Some((PlainParagraph { text: Box::new(x) }, y))
//                } else {
//                    None
//                }
            }
            None => None
        }
    }
    fn heading(&self, pos: usize) -> Option<(Heading, usize)> {
        let rank = self.count(pos, "#");
        if rank < 1 { return None; }
        let pos = self.check_eat(pos, "#", rank);
        let (text, new_pos) = self.text(pos, false, "", true, true, true).unwrap();
        if rank > 6 {
            return None;
        }
        return Some((Heading { rank, text: Box::new(text) }, new_pos));
    }
    fn ordered_list(&self, pos: usize) -> Option<(OrderedList, usize)> {
        let mut pos = pos;
        fn is_item_number(self_: &Parser, pos: usize) -> Option<usize> {
            let mut pos = pos;
            while self_.get_char(pos).unwrap_or(0).is_ascii_digit() {
                pos = self_.eat(pos);
            }
            if self_.is(pos, ". ") {
                return Some(self_.check_eat(pos, ". ", 1));
            }
            return None;
        }
        let mut list = OrderedList { list: vec![] };
        loop {
            match is_item_number(self, pos) {
                Some(p) => pos = p,
                None => break
            }
            match self.text(pos, false, "", true, true, true) {
                Some((x, p)) => {
                    list.list.push(Box::new(x));
                    pos = p;
                }
                None => break
            }
        }
        if list.len() > 0 {
            return Some((list, pos));
        } else {
            return None;
        }
    }
    fn unordered_list(&self, pos: usize) -> Option<(UnorderedList, usize)> {
        let mut pos = pos;
        fn is_item_prefix(self_: &Parser, pos: usize) -> Option<usize> {
            let mut pos = pos;
            let ch = self_.get_char(pos).unwrap_or(0);
            pos = self_.eat(pos);
            for c in "-=*".as_bytes() {
                if ch == *c as InnerByte {
                    if self_.is(pos, " ") {
                        return Some(self_.eat(pos));
                    }
                }
            }
            return None;
        }
        let mut list = UnorderedList { list: vec![] };
        loop {
            match is_item_prefix(self, pos) {
                Some(p) => pos = p,
                None => break
            }
            match self.text(pos, false, "", true, true, true) {
                Some((x, p)) => {
                    list.list.push(Box::new(x));
                    pos = p;
                }
                None => break
            }
        }
        if list.len() > 0 {
            return Some((list, pos));
        } else {
            return None;
        }
    }
    /// todo: does not support complex structural quotes yet
    fn quote(&self, pos: usize) -> Option<(Quote, usize)> {
        let mut pos = pos;
        fn is_quote_prefix(self_: &Parser, pos: usize) -> Option<usize> {
            let mut pos = pos;
            let ch = self_.get_char(pos).unwrap_or(0);
            pos = self_.eat(pos);
            if ch == '>' as InnerByte {
                return Some(self_.eat(pos));
            }
            return None;
        }
        let mut quote = Quote { list: vec![] };
        loop {
            match is_quote_prefix(self, pos) {
                Some(p) => pos = p,
                None => break
            }
            match self.text(pos, false, "", true, true, true) {
                Some((x, p)) => {
                    quote.list.push(Box::new(x));
                    pos = p;
                }
                None => break
            }
        }
        if quote.len() > 0 {
            return Some((quote, pos));
        } else {
            return None;
        }
    }
    fn code_block(&self, pos: usize) -> Option<(CodeBlock, usize)> {
        let mut pos = pos;
        while self.get_char(pos).unwrap_or(48).is_ascii_whitespace() {
            pos += 1;
        }
        if self.is(pos, "```") {
            let mut pos = self.check_eat(pos, "```", 1);
            let lang;
            match self.get_word(pos) {
                Some((s, p)) => {
                    lang = s;
                    pos = p;
                }
                None => lang = vec![]
            }
            let mut text = InnerBuffer::new();
            while pos < self.raw_text.len() {
                if self.is(pos, "```") {
                    pos = self.check_eat(pos, "```", 1);
                    break;
                }
                text.push(self.raw_text[pos]);
                pos = self.eat(pos);
            }
            return Some((CodeBlock { language: lang, text }, pos));
        }
        return None;
    }
    fn math_display(&self, pos: usize) -> Option<(MathDisplay, usize)> {
        if self.is(pos, "$$") {
            let mut pos = self.check_eat(pos, "$$", 1);
            let mut text = InnerBuffer::new();
            while pos < self.raw_text.len() {
                if self.is(pos, "$$") {
                    pos = self.check_eat(pos, "$$", 1);
                    break;
                }
                text.push(self.raw_text[pos]);
                pos = self.eat(pos);
            }
            return Some((MathDisplay { formula: text }, pos));
        }
        return None;
    }
    fn text(&self, pos: usize, linebreak: bool, stop_at: &str, bold: bool, italic: bool, del: bool) -> Option<(Text, usize)> {
        let mut pos = pos;
        let mut text = Text { nodes: vec![] };
        let mut cond = true;
        while cond {
            cond = false;
//            show_slice(&self.raw_text, pos, self.raw_text.len(), pos, pos + 5);
            if self.is(pos, stop_at) {
                pos += stop_at.len();
                break;
            }
            if bold {
                match self.bold(pos, italic, del) {
                    Some((b, p)) => {
                        text.nodes.push(Box::new(b));
                        pos = p;
                        cond = true;
                        continue;
                    }
                    None => {}
                }
            }
            if italic {
                match self.italic(pos, bold, del) {
                    Some((i, p)) => {
                        text.nodes.push(Box::new(i));
                        pos = p;
                        cond = true;
                        continue;
                    }
                    None => {}
                }
            }
            if del {
                match self.deleted(pos, bold, italic) {
                    Some((d, p)) => {
                        text.nodes.push(Box::new(d));
                        pos = p;
                        cond = true;
                        continue;
                    }
                    None => {}
                }
            }
            match self.code_inline(pos) {
                Some((c, p)) => {
                    text.nodes.push(Box::new(c));
                    pos = p;
                    cond = true;
                    continue;
                }
                None => {}
            }
            match self.math_inline(pos) {
                Some((m, p)) => {
                    text.nodes.push(Box::new(m));
                    pos = p;
                    cond = true;
                    continue;
                }
                None => {}
            }
            match self.get_char(pos) {
                Some(c) => {
                    if !linebreak && c == '\n' as InnerByte {
                        pos += 1;
                        break;
                    }

                    text.nodes.push(Box::new(c));
                    pos += 1;
                    cond = true;
                }
                None => {}
            }
        }

        return Some((text, pos));
    }
    fn bold(&self, pos: usize, italic: bool, del: bool) -> Option<(Bold, usize)> {
        if self.is(pos, "**") {
            let pos = self.check_eat(pos, "**", 1);
            match self.text(pos, false, "**", false, italic, del) {
                Some((x, y)) => {
                    if x.len() > 0 {
                        return Some((Bold { text: Box::new(x) }, y));
                    }
                }
                None => {}
            }
        }
        return None;
    }
    fn italic(&self, pos: usize, bold: bool, del: bool) -> Option<(Italic, usize)> {
        if self.is(pos, "*") {
            let pos = self.check_eat(pos, "*", 1);
            match self.text(pos, false, "*", bold, false, del) {
                Some((x, y)) => {
                    if x.len() > 0 {
                        return Some((Italic { text: Box::new(x) }, y));
                    }
                }
                None => {}
            }
        }
        return None;
    }
    fn deleted(&self, pos: usize, bold: bool, italic: bool) -> Option<(Deleted, usize)> {
        if self.is(pos, "~~") {
            let pos = self.check_eat(pos, "~~", 1);
            match self.text(pos, false, "~~", bold, italic, false) {
                Some((x, y)) => {
                    if x.len() > 0 {
                        return Some((Deleted { text: Box::new(x) }, y));
                    }
                }
                None => {}
            }
        }
        return None;
    }

    fn code_inline(&self, pos: usize) -> Option<(CodeInline, usize)> {
        if self.is(pos, "`") {
            let mut pos = self.check_eat(pos, "`", 1);
            let mut text = InnerBuffer::new();
            while pos < self.raw_text.len() {
                if self.is(pos, "\n") || self.is(pos, "`") {
                    pos = self.eat(pos);
                    break;
                }
                text.push(self.raw_text[pos]);
                pos = self.eat(pos);
            }
            if text.len() > 0 {
                return Some((CodeInline { code: text }, pos));
            }
        }
        return None;
    }
    fn math_inline(&self, pos: usize) -> Option<(MathInline, usize)> {
        if self.is(pos, "$") {
            let mut pos = self.check_eat(pos, "$", 1);
            let mut text = InnerBuffer::new();
            while pos < self.raw_text.len() {
                if self.is(pos, "\n") || self.is(pos, "$") {
                    pos = self.eat(pos);
                    break;
                }
                text.push(self.raw_text[pos]);
                pos = self.eat(pos);
            }
            return Some((MathInline { formula: text }, pos));
        }
        return None;
    }
}