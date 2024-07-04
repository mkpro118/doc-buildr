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
