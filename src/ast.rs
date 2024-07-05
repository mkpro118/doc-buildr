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
    comment: Option<&'a entity::DocComment>,
    value: Option<NodeTypes<'a>>,
}

#[derive(Debug)]
pub struct AST<'a> {
    ast: Vec<Node<'a>>,
}

impl<'a> Node<'a> {
    pub fn from(token: &'a ParsedToken, comment: Option<&'a entity::DocComment>) -> Self {
        Self {
            comment,
            value: match token {
                ParsedToken::DocComment(_) => None,
                ParsedToken::Struct(x) => Some(NodeTypes::Struct(x)),
                ParsedToken::Function(x) => Some(NodeTypes::Function(x)),
                ParsedToken::Enum(x) => Some(NodeTypes::Enum(x)),
            },
        }
    }

    pub fn get_comment(&self) -> Option<&entity::DocComment> {
        self.comment
    }

    pub fn get_value(&self) -> &Option<NodeTypes> {
        &self.value
    }
}

impl<'a> AST<'a> {
    pub fn build_ast(parsed_tokens: &'a Vec<ParsedToken>) -> Self {
        let mut ast = vec![];
        let mut current_doc: Option<&entity::DocComment> = None;

        for token in parsed_tokens {
            match token {
                ParsedToken::DocComment(comment) => {
                    current_doc = Some(comment);
                }
                _ => {
                    ast.push(Node::from(token, current_doc));
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
