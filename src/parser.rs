//! # Parser Module
//!
//! This module is responsible for parsing tokens into more structured
//! representations of code elements.

use crate::entity;
use crate::token;
use crate::token::TokenValuePair;
use regex::*;

/// Represents the different types of parsed tokens.
#[derive(Debug)]
pub enum ParsedToken {
    DocComment(entity::DocComment),
    Struct(entity::Struct),
    Function(entity::Function),
    Enum(entity::Enum),
}

/// Attempts to match a regular expression pattern against a source string.
fn get_capture<'a, 'b>(pat: &'a str, src: &'b str) -> Option<Captures<'b>> {
    let re = RegexBuilder::new(pat)
        .dot_matches_new_line(true)
        .build()
        .unwrap();

    re.captures(src)
}

/// Splits a source string by a separator character, trimming whitespace.
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

/// A trait for types that can be parsed from a string.
pub trait Parse: 'static {
    /// Attempts to parse an instance of Self from a string.
    fn parse(src: &str) -> Option<Self>
    where
        Self: Sized;
}

impl Parse for entity::DocComment {
    fn parse(src: &str) -> Option<Self> {
        static PARAM_PATTERN: &str =
            r"[^\S\r\n]*@param[^\S\r\n]+(?<name>\w+)[^\S\r\n]+(?<desc>([^\r\n]+))";

        static RETVAL_PATTERN: &str = r"[^\S\r\n]*@return[^\S\r\n]+(?<desc>([^\r\n])+)";

        enum Section {
            Description,
            Param,
            RetVal,
        }
        use Section::*;

        let mut curr_section = Description;

        let param_re = regex::Regex::new(PARAM_PATTERN).unwrap();
        let retval_re = regex::Regex::new(RETVAL_PATTERN).unwrap();

        let (comment, params, retval) = src
            .strip_prefix("/**")
            .unwrap()
            .strip_suffix("*/")
            .unwrap()
            .split(&['\n', '\r'])
            .map(str::trim)
            .map(|x| x.trim_start_matches('*'))
            .map(str::trim_end)
            .filter(|x| !x.is_empty())
            .fold(
                (String::new(), vec![], None),
                |(mut desc, mut params, mut ret), s| {
                    if let Some(capture) = param_re.captures(s) {
                        curr_section = Param;

                        let name = capture["name"].to_string();
                        let description = capture["desc"].to_string();

                        params.push(entity::Param { name, description });
                    } else if let Some(capture) = retval_re.captures(s) {
                        curr_section = RetVal;

                        let description = capture["desc"].to_string();

                        ret = Some(entity::Return { description });
                    } else {
                        match curr_section {
                            Description => {
                                desc.push_str(s);
                                desc.push_str("\n");
                            }
                            Param => {
                                let Some(param) = params.last_mut() else {
                                panic!("Control should not have reached here.");
                            };
                                param.description.push(' ');
                                param.description.push_str(s.trim());
                            }
                            RetVal => {
                                let Some(ref mut retval) = ret else {
                                panic!("Control should not have reached here.");
                            };
                                retval.description.push(' ');
                                retval.description.push_str(s.trim());
                            }
                        }
                    }

                    (desc, params, ret)
                },
            );

        Some(Self {
            comment,
            params,
            retval,
        })
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

/// Parses a vector of TokenValuePairs into a vector of ParsedTokens.
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
