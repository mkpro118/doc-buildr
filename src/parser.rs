use regex::{*};
use crate::token;
use crate::TokenValuePair;

#[derive(Debug)]
pub struct DocComment {
    pub comment: String
}

#[derive(Debug)]
pub struct Struct {
    pub name: String,
    pub members: Vec<String>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub return_type: String,
    pub params: Vec<String>,
}

#[derive(Debug)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<String>,
}

pub enum ParsedToken {
    DocComment(DocComment),
    Struct(Struct),
    Function(Function),
    Enum(Enum),
}

fn get_capture<'a, 'b>(
    pat: &'a str,
    src: &'b str
) -> Option<Captures<'b>> {
    let re = RegexBuilder::new(pat)
        .dot_matches_new_line(true)
        .build()
        .unwrap();

    re.captures(src)
}

fn src_split(source: &str, sep: char) -> Vec<String> {
    source.split(sep)
          .map(str::trim)
          .filter_map(|x| if x.is_empty() {None} else {Some(String::from(x))})
          .collect::<Vec<String>>()
}

pub trait Parse: 'static  {
    fn parse(src: &str) -> Option<Self> where Self: Sized;
}


impl Parse for DocComment {
    fn parse(src: &str) -> Option<Self> {
        let comment = src
            .strip_prefix("/**").unwrap()
            .strip_suffix("*/").unwrap()
            .split(&['\n', '\r'])
            .map(str::trim)
            .map(|x| x.trim_start_matches('*'))
            .map(str::trim)
            .collect::<Vec<_>>()
            .join("\n");

        Some(Self {comment})
    }
}

impl Parse for Struct {
    fn parse(src: &str) -> Option<Self> {
        static PAT: &'static str = r"struct\s+(\w+)\s*\{(.*?)\}";
        let Some(capture) = get_capture(PAT, src) else { return None; };

        let (_, [name, members]) = capture.extract();

        let name = String::from(name);
        let members = src_split(members, ';');

        Some(Self {name, members})
    }
}

impl Parse for Function {
    fn parse(src: &str) -> Option<Self> {
        static PAT: &'static str = r"(\w+)\s+(\w+)\s*\((.*?)\)";
        let Some(capture) = get_capture(PAT, src) else { return None; };

        let (_, [return_type, name, params]) = capture.extract();

        let name = String::from(name);
        let return_type = String::from(return_type);
        let params = src_split(params, ',');

        Some(Self {name, return_type, params})
    }
}

impl Parse for Enum {
    fn parse(src: &str) -> Option<Self> {
        static PAT: &'static str =r"enum\s+(\w+)\s*\{(.*?)\}";
        let Some(capture) = get_capture(PAT, src) else { return None; };

        let (_, [name, variants]) = capture.extract();

        let name = String::from(name);
        let variants = src_split(variants, ',');

        Some(Self {name, variants})
    }
}

pub fn parse_tokens(pairs: Vec<TokenValuePair>) -> Vec<ParsedToken> {
    pairs.iter().filter_map(|pair|  Some(match pair.token {
        token::DocComment => ParsedToken::DocComment(
            DocComment::parse(&pair.value).unwrap()
        ),
        token::Enum => ParsedToken::Enum(
            Enum::parse(&pair.value).unwrap()
        ),
        token::Function => ParsedToken::Function(
            Function::parse(&pair.value).unwrap()
        ),
        token::Struct => ParsedToken::Struct(
            Struct::parse(&pair.value).unwrap()
        ),
    })).collect()
}
