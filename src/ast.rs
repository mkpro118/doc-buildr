use crate::entity;
use crate::parser::*;

#[derive(Debug)]
pub enum NodeTypes<'a> {
    Enum(&'a entity::Enum),
    Function(&'a entity::Function),
    Struct(&'a entity::Struct),
}

#[derive(Debug)]
pub struct Node<'a> {
    comment: String,
    value: Option<NodeTypes<'a>>,
}

#[derive(Debug)]
pub struct AST<'a> {
    ast: Vec<Node<'a>>,
}

impl<'a> Node<'a> {
    pub fn from(token: &'a ParsedToken, comment: &str) -> Self {
        Self {
            comment: comment.to_string(),
            value: match token {
                ParsedToken::DocComment(_) => None,
                ParsedToken::Struct(x) => Some(NodeTypes::Struct(x)),
                ParsedToken::Function(x) => Some(NodeTypes::Function(x)),
                ParsedToken::Enum(x) => Some(NodeTypes::Enum(x)),
            },
        }
    }

    pub fn get_comment(&self) -> &String {
        &self.comment
    }

    pub fn get_value(&self) -> &Option<NodeTypes> {
        &self.value
    }
}

impl<'a> AST<'a> {
    pub fn build_ast(parsed_tokens: &'a Vec<ParsedToken>) -> Self {
        let default_comment: String = String::from("No documentation available");
        let mut ast = vec![];
        let mut current_doc: Option<String> = None;

        for token in parsed_tokens {
            match token {
                ParsedToken::DocComment(comment) => {
                    current_doc = Some(comment.comment.to_owned());
                }
                _ => {
                    ast.push(Node::from(
                        token,
                        &current_doc.unwrap_or_else(|| default_comment.clone()),
                    ));
                    current_doc = None;
                }
            };
        }

        Self { ast }
    }

    pub fn get_iter(&self) -> std::slice::Iter<'_, Node> {
        self.ast.iter()
    }
}
