use crate::entity;
use crate::parser::{*};

#[derive(Debug)]
pub enum NodeTypes {
    Enum(entity::Enum),
    Function(entity::Function),
    Struct(entity::Struct),
}

#[derive(Debug)]
pub struct Node {
    comment: String,
    value: Option<NodeTypes>
}

impl Node {
    pub fn from(token: ParsedToken, comment: &str) -> Self {
        Self {
            comment: comment.to_string(),
            value: match token {
                ParsedToken::DocComment(_) => None,
                ParsedToken::Struct(x) => Some(NodeTypes::Struct(x)),
                ParsedToken::Function(x) => Some(NodeTypes::Function(x)),
                ParsedToken::Enum(x) => Some(NodeTypes::Enum(x)),
            }
        }
    }
}

pub fn build_ast(parsed_tokens: Vec<ParsedToken>) -> Vec<Node> {
    let default_comment: String = String::from("No documentation available");
    let mut ast = vec![];
    let mut current_doc: Option<String> = None;

    for token in parsed_tokens {
        match token {
            ParsedToken::DocComment(comment) => {
                current_doc = Some(comment.comment.to_owned());
            },
            _ => {
                ast.push(Node::from(token,
                    &current_doc.unwrap_or_else(|| default_comment.clone())));
                current_doc = None;
            },
        };
    }

    ast
}
