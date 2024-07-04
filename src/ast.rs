use crate::parser;
use crate::parser::{*};

#[derive(Debug)]
enum NodeTypes {
    Enum(parser::Enum),
    Function(parser::Function),
    Struct(parser::Struct),
}

#[derive(Debug)]
struct Node<'a> {
    comment: &'a String,
    children: Vec<NodeTypes>,
    value: Option<NodeTypes>
}

impl<'a> Node<'a> {
    pub fn from(token: ParsedToken, comment: &'a String) -> Self {
        Self {
            comment: comment,
            children: vec![],
            value: match token {
                ParsedToken::DocComment(_) => None,
                ParsedToken::Struct(x) => Some(NodeTypes::Struct(x)),
                ParsedToken::Function(x) => Some(NodeTypes::Function(x)),
                ParsedToken::Enum(x) => Some(NodeTypes::Enum(x)),
            }
        }
    }
}
