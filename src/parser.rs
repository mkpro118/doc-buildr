use regex::{*};
use crate::token::{*};

#[derive(Debug)]
struct DocComment {
    comment: String
}

#[derive(Debug)]
struct Struct {
    name: String,
    members: Vec<String>,
}

#[derive(Debug)]
struct Function {
    name: String,
    return_type: String,
    params: Vec<String>,
}

#[derive(Debug)]
struct Enum {
    name: String,
    variants: Vec<String>,
}

trait Parse {
    fn parse(pair: &TokenValuePair) -> Option<Self> where Self: Sized;
}

fn src_split(source: &str, sep: char) -> Vec<String> {
    source.split(sep)
          .map(str::trim)
          .filter_map(|x| if x.is_empty() {None} else {Some(String::from(x))})
          .collect::<Vec<String>>()
}

impl Parse for DocComment {
    fn parse(pair: &TokenValuePair) -> Option<Self> {
        let comment = pair.value
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
    fn parse(pair: &TokenValuePair) -> Option<Self> {
        let re = RegexBuilder::new(r"struct\s+(\w+)\s*\{(.*?)\}")
            .dot_matches_new_line(true)
            .build()
            .unwrap();

        let Some(capture) = re.captures(&pair.value) else { return None; };

        let (_, [name, members]) = capture.extract();

        let name = String::from(name);
        let members = src_split(members, ';');

        Some(Self {name, members})
    }
}

impl Parse for Function {
    fn parse(pair: &TokenValuePair) -> Option<Self> {
        let re = RegexBuilder::new(r"(\w+)\s+(\w+)\s*\((.*?)\)")
            .dot_matches_new_line(true)
            .build()
            .unwrap();

        let Some(capture) = re.captures(&pair.value) else { return None; };

        let (_, [return_type, name, params]) = capture.extract();

        let name = String::from(name);
        let return_type = String::from(return_type);
        let params = src_split(params, ',');

        Some(Self {name, return_type, params})
    }
}

impl Parse for Enum {
    fn parse(pair: &TokenValuePair) -> Option<Self> {
        let re = RegexBuilder::new(r"enum\s+(\w+)\s*\{(.*?)\}")
            .dot_matches_new_line(true)
            .build()
            .unwrap();

        let Some(capture) = re.captures(&pair.value) else { return None; };

        let (_, [name, variants]) = capture.extract();

        let name = String::from(name);
        let variants = src_split(variants, ',');

        Some(Self {name, variants})
    }
}
