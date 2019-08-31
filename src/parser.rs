trait Node {
    //    fn get_node_type(&self) -> &str;
    fn get_string(&self) -> String;
}

trait Paragraph {}

trait Text {}

struct Passage {
    paragraphs: Vec<Box<dyn Paragraph>>
}


struct PlainParagraph {
    text: Box<dyn Text>
}


struct Heading {
    heading_level: i32,
    text: Box<dyn Text>,
}

struct OrderedList {
    text: Box<dyn Text>
}

struct UnorderedList {
    text: Box<dyn Text>
}

struct Quote {
    quote_level: i32,
    text: Box<dyn Text>,
}

struct CodeBlock {
    language: String,
    text: String,
}

struct Bold {
    text: Box<dyn Text>
}

struct MathDisplay {
    formula: String
}

struct Italic {
    text: Box<dyn Text>
}

struct Deleted {
    text: Box<dyn Text>
}

struct CodeInline {
    code: String
}

struct MathInline {
    formula: String
}


impl Paragraph for PlainParagraph {}

impl Paragraph for Heading {}

impl Paragraph for OrderedList {}

impl Paragraph for UnorderedList {}

impl Paragraph for Quote {}

impl Paragraph for CodeBlock {}

impl Paragraph for MathDisplay {}

impl Text for Bold {}

impl Text for Italic {}

impl Text for Deleted {}

impl Text for CodeInline {}

impl Text for MathInline {}

impl Text for u8 {}

impl Node for PlainParagraph {
    fn get_string(&self) -> String {
        unimplemented!()
    }
}


impl Node for Heading {
    fn get_string(&self) -> String {
        unimplemented!()
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
        unimplemented!()
    }
}


impl Node for MathDisplay {
    fn get_string(&self) -> String {
        unimplemented!()
    }
}


impl Node for Bold {
    fn get_string(&self) -> String {
        unimplemented!()
    }
}


impl Node for Italic {
    fn get_string(&self) -> String {
        unimplemented!()
    }
}


impl Node for Deleted {
    fn get_string(&self) -> String {
        unimplemented!()
    }
}


impl Node for CodeInline {
    fn get_string(&self) -> String {
        unimplemented!()
    }
}


impl Node for MathInline {
    fn get_string(&self) -> String {
        unimplemented!()
    }
}

impl Node for u8 {
    fn get_string(&self) -> String {
        unimplemented!()
    }
}

struct Parser {
    raw_text: Vec<u8>,
}

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
        unimplemented!();
    }
    fn get_word(&self, pos: usize) -> Option<(String, usize)> {
        unimplemented!();
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
    fn parse(&self) -> String { unimplemented!() }
    fn passage(&self, pos: usize) -> Option<(Passage, usize)> { unimplemented!() }
    fn paragraph(&self, pos: usize) -> Option<(Box<dyn Paragraph>, usize)> { unimplemented!() }
    fn plain(&self, pos: usize) -> Option<(PlainParagraph, usize)> { unimplemented!() }
    fn heading(&self, pos: usize) -> Option<(Heading, usize)> { unimplemented!() }
    fn ordered_list(&self, pos: usize) -> Option<(OrderedList, usize)> { unimplemented!() }
    fn unordered_list(&self, pos: usize) -> Option<(UnorderedList, usize)> { unimplemented!() }
    fn quote(&self, pos: usize) -> Option<(Quote, usize)> { unimplemented!() }
    fn code_block(&self, pos: usize) -> Option<(CodeBlock, usize)> { unimplemented!() }
    fn math_display(&self, pos: usize) -> Option<(MathDisplay, usize)> { unimplemented!() }
    fn text(&self, pos: usize) -> Option<(Box<dyn Text>, usize)> { unimplemented!() }

    fn bold(&self, pos: usize) -> Option<(Bold, usize)> { unimplemented!() }
    fn italic(&self, pos: usize) -> Option<(Italic, usize)> { unimplemented!() }
    fn deleted(&self, pos: usize) -> Option<(Deleted, usize)> { unimplemented!() }
    fn code_inline(&self, pos: usize) -> Option<(CodeInline, usize)> { unimplemented!() }
    fn math_inline(&self, pos: usize) -> Option<(MathInline, usize)> { unimplemented!() }
}