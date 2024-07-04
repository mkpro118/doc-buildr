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
    fn parse(pair: &TokenValuePair) -> Self;
}

impl Parse for DocComment {
    fn parse(pair: &TokenValuePair) -> Self {
        let content = pair.value
            .strip_prefix("/**").unwrap()
            .strip_suffix("*/").unwrap()
            .split(&['\n', '\r'])
            .map(str::trim)
            .map(|x| x.trim_start_matches('*'))
            .map(str::trim)
            .collect::<Vec<_>>()
            .join("\n");

        Self {comment: content}
    }
}
