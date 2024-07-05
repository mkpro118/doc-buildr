use crate::entity;
use crate::token;
use crate::token::TokenValuePair;
use regex::*;

#[derive(Debug)]
pub enum ParsedToken {
    DocComment(entity::DocComment),
    Struct(entity::Struct),
    Function(entity::Function),
    Enum(entity::Enum),
}

fn get_capture<'a, 'b>(pat: &'a str, src: &'b str) -> Option<Captures<'b>> {
    let re = RegexBuilder::new(pat)
        .dot_matches_new_line(true)
        .build()
        .unwrap();

    re.captures(src)
}

fn src_split(source: &str, sep: char) -> Vec<String> {
    source
        .split(sep)
        .map(str::trim)
        .filter_map(|x| {
            if x.is_empty() {
                None
            } else {
                Some(String::from(x))
            }
        })
        .collect::<Vec<String>>()
}

pub trait Parse: 'static {
    fn parse(src: &str) -> Option<Self>
    where
        Self: Sized;
}

impl Parse for entity::DocComment {
    fn parse(src: &str) -> Option<Self> {
        let comment = src
            .strip_prefix("/**")
            .unwrap()
            .strip_suffix("*/")
            .unwrap()
            .split(&['\n', '\r'])
            .map(str::trim)
            .map(|x| x.trim_start_matches('*'))
            .map(str::trim_end)
            .filter(|x| !x.is_empty())
            .map(|x| {
                let mut s = x.to_string();
                s.push('\\');
                s
            })
            .collect::<Vec<_>>()
            .join("\n");

        Some(Self { comment })
    }
}

impl Parse for entity::Struct {
    fn parse(src: &str) -> Option<Self> {
        static PAT: &'static str = r"struct\s+(\w+)\s*\{(.*?)\}";
        let Some(capture) = get_capture(PAT, src) else { return None; };

        let (_, [name, members]) = capture.extract();

        let name = String::from(name);
        let members = src_split(members, ';');

        Some(Self { name, members })
    }
}

impl Parse for entity::Function {
    fn parse(src: &str) -> Option<Self> {
        static PAT: &'static str = r"(\w+)\s+(\w+)\s*\((.*?)\)";
        let Some(capture) = get_capture(PAT, src) else { return None; };

        let (_, [return_type, name, params]) = capture.extract();

        let name = String::from(name);
        let return_type = String::from(return_type);
        let params = src_split(params, ',');

        Some(Self {
            name,
            return_type,
            params,
        })
    }
}

impl Parse for entity::Enum {
    fn parse(src: &str) -> Option<Self> {
        static PAT: &'static str = r"enum\s+(\w+)\s*\{(.*?)\}";
        let Some(capture) = get_capture(PAT, src) else { return None; };

        let (_, [name, variants]) = capture.extract();

        let name = String::from(name);
        let variants = src_split(variants, ',');

        Some(Self { name, variants })
    }
}

pub fn parse_tokens(pairs: &Vec<TokenValuePair>) -> Vec<ParsedToken> {
    pairs
        .iter()
        .filter_map(|pair| {
            Some(match pair.token {
                token::DocComment => {
                    ParsedToken::DocComment(entity::DocComment::parse(&pair.value).unwrap())
                }
                token::Enum => ParsedToken::Enum(entity::Enum::parse(&pair.value).unwrap()),
                token::Function => {
                    ParsedToken::Function(entity::Function::parse(&pair.value).unwrap())
                }
                token::Struct => ParsedToken::Struct(entity::Struct::parse(&pair.value).unwrap()),
            })
        })
        .collect()
}
