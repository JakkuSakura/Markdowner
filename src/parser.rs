use std::any::{Any, TypeId};

trait Node: Any {
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

struct Image {
    alt: InnerBuffer,
    url: InnerBuffer,
    title: InnerBuffer,
}

struct Url {
    text: InnerBuffer,
    url: InnerBuffer,
    title: InnerBuffer,
}

impl Node for Passage {
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
    fn write_to_buf(&self, buf: &mut dyn Buf) {
        buf.push_str("<pre><code>");
        buf.push_vec(&self.code);
        buf.push_str("</code><pre>");
    }

    fn len(&self) -> i32 {
        self.code.len() as i32
    }
}


impl Node for MathInline {
    fn write_to_buf(&self, buf: &mut dyn Buf) {
        buf.push_str("<div class=\"lang-math-inline\">");
        buf.push_vec(&self.formula);
        buf.push_str("</div>");
    }

    fn len(&self) -> i32 {
        self.formula.len() as i32
    }
}

impl Node for Image {
    fn write_to_buf(&self, buf: &mut dyn Buf) {
        buf.push_str("<img src=\"");
        buf.push_vec(&self.url);
        buf.push_str("\" alt=\"");
        buf.push_vec(&self.alt);
        buf.push_str("\" title=\"");
        buf.push_vec(&self.title);
        buf.push_str("\">");
    }

    fn len(&self) -> i32 {
        (self.alt.len() + self.url.len() + self.title.len()) as i32
    }
}

impl Node for Url {
    fn write_to_buf(&self, buf: &mut dyn Buf) {
        buf.push_str("<a href=\"");
        buf.push_vec(&self.url);
        buf.push_str("\" title=\"");
        buf.push_vec(&self.title);
        buf.push_str("\">");
        buf.push_vec(&self.text);
        buf.push_str("</a>");
    }

    fn len(&self) -> i32 {
        (self.text.len() + self.url.len() + self.title.len()) as i32
    }
}

impl Node for InnerByte {
    fn write_to_buf(&self, buf: &mut dyn Buf) {
        buf.push(*self);
    }

    fn len(&self) -> i32 {
        1
    }
}

fn encase<T: Node + 'static>(x: Option<(T, usize)>) -> Option<(Box<dyn Node>, usize)> {
    x.map(|(y, z)| (Box::new(y) as _, z))
}

mod buf_util {
    use super::InnerBuffer;

    fn trim_left(buf: &mut InnerBuffer) {
        let mut j = 0;
        let mut first = true;
        for i in 0..buf.len() {
            if !first || !buf[i].is_ascii_whitespace() {
                buf[j] = buf[i];
                j += 1;
                first = false;
            }
        }
        for i in j..buf.len() {
            buf.pop();
        }
    }

    fn trim_right(buf: &mut InnerBuffer) {
        for i in (0..buf.len()).rev() {
            if buf[i].is_ascii_whitespace() {
                buf.pop();
            } else {
                break;
            }
        }
    }

    fn trim(buf: &mut InnerBuffer) {
        trim_right(buf);
        trim_left(buf);
    }

    pub(crate) fn is(text: &InnerBuffer, pos: usize, s: &str) -> bool {
        if s == "" { return false; }
        if pos + s.len() > text.len() {
            return false;
        }
        let xx = s.as_bytes();
        for i in 0..xx.len() {
            if text[pos + i] != xx[i] {
                return false;
            }
        }
        return true;
    }

    pub(crate) fn get_word(text: &InnerBuffer, pos: usize) -> Option<(InnerBuffer, usize)> {
        if !text[pos].is_ascii_alphanumeric() {
            return None;
        }
        let mut num: InnerBuffer = vec![];
        let mut pos = pos;
        while pos < text.len() {
            if !text[pos].is_ascii_alphanumeric() {
                break;
            }
            num.push(text[pos]);
            pos += 1;
        }
        return Some((num, pos));
    }

    pub(crate) fn get_until_pattern(text: &InnerBuffer, pos: usize, until: impl Fn(usize) -> Option<usize>, new_line: bool) -> GetUntilResult {
        let mut buf = InnerBuffer::new();
        let mut pos = pos;
        while pos < text.len() {
            if !new_line && is(&text, pos, "\n") {
                pos = check_eat(&text, pos, "\n", 1);
                return GetUntilResult {
                    text: buf,
                    pos,
                    reached_target: false,
//                    die_of_newline: true,
//                    die_of_eof: false,
                };
            }
            match until(pos) {
                Some(p) => {
                    pos = p;
                    return GetUntilResult {
                        text: buf,

                        pos,
                        reached_target: true,
//                        die_of_newline: false,
//                        die_of_eof: false,
                    };
                }
                None => {}
            }
            buf.push(text[pos]);
            pos += 1;
        }
        return GetUntilResult {
            text: buf,
            pos,
            reached_target: false,
//            die_of_newline: false,
//            die_of_eof: true,
        };
    }

    pub(crate) fn get_until(text: &InnerBuffer, pos: usize, until: &str, multi_lines: bool) -> GetUntilResult {
        let judge = |pz: usize| {
            if is(&text, pz, until) {
                Some(check_eat(&text, pz, until, 1))
            } else {
                None
            }
        };
        get_until_pattern(&text, pos, judge, multi_lines)
    }

    use super::*;
    pub(crate) fn character(text: &InnerBuffer, pos: usize) -> InnerByte {
        if pos < text.len() {
            let ch = text[pos];

            // REPLACEMENT CHARACTER according to GFM
            // if InnerByte less than 2 bytes, maybe this will not work
            if ch == 0 as InnerByte {
                if TypeId::of::<InnerByte>() == TypeId::of::<char>() ||
                    TypeId::of::<InnerByte>() == TypeId::of::<i32>() ||
                    TypeId::of::<InnerByte>() == TypeId::of::<u32>() ||
                    TypeId::of::<InnerByte>() == TypeId::of::<i16>() ||
                    TypeId::of::<InnerByte>() == TypeId::of::<u16>()
                {
                    return '\u{FFFD}' as InnerByte;
                }
            }
            return ch;
        }
        return (-1i32) as InnerByte; // EOF
    }

    pub(crate) fn is_eof(text: &InnerBuffer, pos: usize) -> bool {
        character(&text, pos) == (-1i32) as InnerByte
    }

    /// returns 1 if a newline (U+000A)
    /// returns 1 if a carriage return (U+000D) not followed by a newline
    /// or returns 2 if a carriage return and a following newline.
    fn is_line_ending(text: &InnerBuffer, pos: usize) -> i32 {
        if character(&text, pos) == '\n' as InnerByte {
            return 1;
        }
        if character(&text, pos) == '\r' as InnerByte {
            if character(&text, pos + 1) == '\n' as InnerByte {
                return 2;
            }
            return 1;
        }
        return 0;
    }

    pub(crate) fn check_eat(text: &InnerBuffer, pos: usize, s: &str, cnt: i32) -> usize {
        if s != "" {
            if count(text, pos, s) < cnt {
                panic!("Cannot eat {} for {} time(s)", s, cnt);
            }
            return pos + s.len() * cnt as usize;
        } else {
            return pos + cnt as usize;
        }
    }

    pub(crate) fn count(text: &InnerBuffer, pos: usize, s: &str) -> i32 {
        let mut cnt = 0;
        let mut p = pos;
        while p + s.len() <= text.len() {
            if is(&text, p, s) {
                p += s.len();
                cnt += 1;
            } else { break; }
        }
        return cnt;
    }

    pub struct GetUntilResult {
        pub(crate) text: InnerBuffer,
        pub(crate) pos: usize,
        pub(crate) reached_target: bool,
//    die_of_newline: bool,
//    die_of_eof: bool,
    }
}


pub fn parse(input: &InnerBuffer, buf: &mut dyn Buf) {
    match passage(input, 0) {
        Some((p, _)) => {
//                Parser::readjust(&mut p);
            p.write_to_buf(buf);
        }
        None => panic!("unexpected error: unknown reason")
    }
}

//    fn readjust(node: &mut dyn Node) {
//        fn is<T0: ?Sized + Any, T: ?Sized + Any>(_s: &T) -> bool {
//            TypeId::of::<T0>() == TypeId::of::<T>()
//        }
//
//        if is::<OrderedList, _>(node) {
//            println!("Node is ordered list");
//        }
//    }
fn passage(text: &InnerBuffer, pos: usize) -> Option<(Passage, usize)> {
    let mut pos = pos;
    let mut psg = Passage { paragraphs: vec![] };
    while pos < text.len() {
        match paragraph(&text, pos) {
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

fn paragraph(text: &InnerBuffer, pos: usize) -> Option<(Box<dyn Node>, usize)> {
    let heading = heading(&text, pos);
    if heading.is_some() { return encase(heading); }
    let ordered_list = ordered_list(&text, pos);
    if ordered_list.is_some() { return encase(ordered_list); }

    let unordered_list = unordered_list(&text, pos);
    if unordered_list.is_some() { return encase(unordered_list); }

    let quote = quote(&text, pos);
    if quote.is_some() { return encase(quote); }

    let code_block = code_block(&text, pos);
    if code_block.is_some() { return encase(code_block); }

    let math_display = math_display(&text, pos);
    if math_display.is_some() { return encase(math_display); }

    let plain_paragraph = plain_paragraph(&text, pos);
    if plain_paragraph.is_some() { return encase(plain_paragraph); }

    return None;
}

fn plain_paragraph(text: &InnerBuffer, pos: usize) -> Option<(PlainParagraph, usize)> {
    match inline(text, pos, false, "", true, true, true) {
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
use buf_util::*;
fn heading(text: &InnerBuffer, pos: usize) -> Option<(Heading, usize)> {
    let rank = count(&text, pos, "#");
    if rank < 1 { return None; }
    let pos = check_eat(&text, pos, "#", rank);
    let (text, new_pos) = inline(text, pos, false, "", true, true, true).unwrap();
    if rank > 6 {
        return None;
    }
    return Some((Heading { rank, text: Box::new(text) }, new_pos));
}

fn ordered_list(text: &InnerBuffer, pos: usize) -> Option<(OrderedList, usize)> {
    let mut pos = pos;
    let is_item_number = |pz: usize| -> Option<usize> {
        let mut pz = pz;
        while character(&text, pz).is_ascii_digit() {
            pz += 1;
        }
        if is(&text, pz, ". ") {
            return Some(check_eat(&text, pz, ". ", 1));
        }
        return None;
    };
    let mut list = OrderedList { list: vec![] };
    loop {
        match is_item_number(pos) {
            Some(p) => pos = p,
            None => break
        }
        match inline(text, pos, false, "", true, true, true) {
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

fn unordered_list(text: &InnerBuffer, pos: usize) -> Option<(UnorderedList, usize)> {
    let mut pos = pos;
    let is_item_prefix = |pz: usize| -> Option<usize> {
        let mut pz = pz;
        let ch = character(&text, pz);
        pz += 1;
        for c in "-=*".as_bytes() {
            if ch == *c as InnerByte {
                if is(&text, pz, " ") {
                    return Some(pz + 1);
                }
            }
        }
        return None;
    };
    let mut list = UnorderedList { list: vec![] };
    loop {
        match is_item_prefix(pos) {
            Some(p) => pos = p,
            None => break
        }
        match inline(&text, pos, false, "", true, true, true) {
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
fn quote(text: &InnerBuffer, pos: usize) -> Option<(Quote, usize)> {
    let mut pos = pos;
    let is_quote_prefix = |pz: usize| -> Option<usize> {
        let mut pz = pz;
        let ch = character(&text, pz);
        pz += 1;
        if ch == '>' as InnerByte {
            return Some(pz + 1);
        }
        return None;
    };
    let mut quote = Quote { list: vec![] };
    loop {
        match is_quote_prefix(pos) {
            Some(p) => pos = p,
            None => break
        }
        match inline(text, pos, false, "", true, true, true) {
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

fn code_block(text: &InnerBuffer, pos: usize) -> Option<(CodeBlock, usize)> {
    let mut pos = pos;
    while character(&text, pos).is_ascii_whitespace() {
        pos += 1;
    }
    if is(&text, pos, "```") {
        let mut pos = check_eat(&text, pos, "```", 1);
        let lang;
        match get_word(&text, pos) {
            Some((s, p)) => {
                lang = s;
                pos = p;
            }
            None => lang = vec![]
        }
        let result = get_until(&text, pos, "```", true);
        return Some((CodeBlock { language: lang, text: result.text }, result.pos));
    }
    return None;
}

fn math_display(text: &InnerBuffer, pos: usize) -> Option<(MathDisplay, usize)> {
    if is(&text, pos, "$$") {
        let pos = check_eat(&text, pos, "$$", 1);
        let result = get_until(&text, pos, "$$", true);
        return Some((MathDisplay { formula: result.text }, result.pos));
    }
    return None;
}

fn inline(text: &InnerBuffer, pos: usize, multi_lines: bool, stop_at: &str, enable_bold: bool, enable_italic: bool, enable_deleted: bool) -> Option<(Text, usize)> {
    let mut pos = pos;
    let mut text_node = Text { nodes: vec![] };
    let mut cond = true;
    while cond {
        cond = false;
//            show_slice(text: &InnerBuffer.raw_text, pos, self.raw_text.len(), pos, pos + 5);
        if is(&text, pos, stop_at) {
            pos += stop_at.len();
            break;
        }
        if enable_bold {
            match bold(&text, pos, enable_italic, enable_deleted) {
                Some((b, p)) => {
                    text_node.nodes.push(Box::new(b));
                    pos = p;
                    cond = true;
                    continue;
                }
                None => {}
            }
        }
        if enable_italic {
            match italic(&text, pos, enable_bold, enable_deleted) {
                Some((i, p)) => {
                    text_node.nodes.push(Box::new(i));
                    pos = p;
                    cond = true;
                    continue;
                }
                None => {}
            }
        }
        if enable_deleted {
            match deleted(&text, pos, enable_bold, enable_italic) {
                Some((d, p)) => {
                    text_node.nodes.push(Box::new(d));
                    pos = p;
                    cond = true;
                    continue;
                }
                None => {}
            }
        }
        match code_inline(&text, pos) {
            Some((c, p)) => {
                text_node.nodes.push(Box::new(c));
                pos = p;
                cond = true;
                continue;
            }
            None => {}
        }
        match math_inline(&text, pos) {
            Some((m, p)) => {
                text_node.nodes.push(Box::new(m));
                pos = p;
                cond = true;
                continue;
            }
            None => {}
        }
        match image(&text, pos) {
            Some((i, p)) => {
                text_node.nodes.push(Box::new(i));
                pos = p;
                cond = true;
                continue;
            }
            None => {}
        }
        match url(&text, pos) {
            Some((u, p)) => {
                text_node.nodes.push(Box::new(u));
                pos = p;
                cond = true;
                continue;
            }
            None => {}
        }
        if !is_eof(&text, pos) {
            let c = character(&text, pos);

            if !multi_lines && c == '\n' as InnerByte {
                pos += 1;
                break;
            }
            text_node.nodes.push(Box::new(c));
            pos += 1;
            cond = true;
        }
    }

    return Some((text_node, pos));
}

fn bold(text: &InnerBuffer, pos: usize, italic: bool, del: bool) -> Option<(Bold, usize)> {
    if is(&text, pos, "**") {
        let pos = check_eat(&text, pos, "**", 1);
        match inline(&text, pos, false, "**", false, italic, del) {
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

fn italic(text: &InnerBuffer, pos: usize, bold: bool, del: bool) -> Option<(Italic, usize)> {
    if is(&text, pos, "*") {
        let pos = check_eat(&text, pos, "*", 1);
        match inline(&text, pos, false, "*", bold, false, del) {
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

fn deleted(text: &InnerBuffer, pos: usize, bold: bool, italic: bool) -> Option<(Deleted, usize)> {
    if is(&text, pos, "~~") {
        let pos = check_eat(&text, pos, "~~", 1);
        match inline(&text, pos, false, "~~", bold, italic, false) {
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

fn code_inline(text: &InnerBuffer, pos: usize) -> Option<(CodeInline, usize)> {
    if is(&text, pos, "`") {
        let pos = check_eat(&text, pos, "`", 1);
        let result = get_until(&text, pos, "`", false);
        if result.text.len() > 0 {
            return Some((CodeInline { code: result.text }, result.pos));
        }
    }
    return None;
}

fn math_inline(text: &InnerBuffer, pos: usize) -> Option<(MathInline, usize)> {
    if is(&text, pos, "$") {
        let pos = check_eat(&text, pos, "$", 1);
        let result = get_until(&text, pos, "`", false);
        if result.text.len() > 0 {
            return Some((MathInline { formula: result.text }, result.pos));
        }
    }
    return None;
}

fn image(text: &InnerBuffer, pos: usize) -> Option<(Image, usize)> {
    let mut pos = pos;

    if !is(&text, pos, "![") {
        return None;
    }
    pos = check_eat(&text, pos, "![", 1);
    let result = get_until(&text, pos, "]", false);
    if !result.reached_target {
        return None;
    }
    let (alt, mut pos) = (result.text, result.pos);

    if !is(&text, pos, "(") {
        return None;
    }
    pos = check_eat(&text, pos, "(", 1);

    let result2 =
        get_until_pattern(&text, pos,
                          |x|
                              if is(&text, x, " ") || is(&text, x, ")") {
                                  Some(x + 1)
                              } else {
                                  None
                              }
                          , false);
    if !result2.reached_target {
        return None;
    }
    let (url, mut pos) = (result2.text, result2.pos - 1);

    let title;
    if is(&text, pos, " ") {
        pos = check_eat(&text, pos, " ", 1);
        let result = get_until(&text, pos, ")", false);
        if !result.reached_target {
            return None;
        }
        title = result.text;
        pos = pos;
    } else {
        title = vec![];
    }

    if !is(&text, pos, ")") {
        return None;
    }

    return Some((Image { alt, url, title }, check_eat(&text, pos, ")", 1)));
}

fn url(text: &InnerBuffer, pos: usize) -> Option<(Url, usize)> {
    let mut pos = pos;

    if !is(&text, pos, "[") {
        return None;
    }
    pos = check_eat(&text, pos, "[", 1);
    let result = get_until(&text, pos, "]", false);
    if !result.reached_target {
        return None;
    }
    let (buf, mut pos) = (result.text, result.pos);
    if !is(&text, pos, "(") {
        return None;
    }
    pos = check_eat(&text, pos, "(", 1);
    let result2 =
        get_until_pattern(&text, pos,
                          |x|
                              if is(&text, x, " ") || is(&text, x, ")") {
                                  Some(x + 1)
                              } else {
                                  None
                              }
                          , false);
    if !result2.reached_target {
        return None;
    }
    let (url, mut pos) = (result2.text, result2.pos - 1);
    let title;
    if is(&text, pos, " ") {
        pos = check_eat(&text, pos, " ", 1);
        let result = get_until(&text, pos, ")", false);
        if !result.reached_target {
            return None;
        }
        title = result.text;
        pos = pos;
    } else {
        title = vec![];
    }
    if !is(&text, pos, ")") {
        return None;
    }

    return Some((Url { text: buf, url, title }, check_eat(text, pos, ")", 1)));
}


#[cfg(test)]
mod tests {
    use super::*;

    fn easy_parse(s: &str) -> String {
        let input = s.to_string();
        let mut buf: Vec<u8> = vec![];
        parse(&input.into_bytes(), &mut buf);
        return String::from_utf8(buf).unwrap();
    }

    fn test(raw: &str, fnl: &str) {
        let rlt = easy_parse(raw);
        if &rlt != fnl {
            panic!("Test failed. \nInput:\n{}\nResult:\n{}\nShould be:\n{}\n",
                   raw, rlt, fnl
            )
        }
    }
    #[test]
    fn start_test(){}

    include!("gfm_tests.in");
}