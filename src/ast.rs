//! # Abstract Syntax Tree (AST) Module
//!
//! This module defines the structure and implementation of the Abstract Syntax Tree
//! used in doc-buildr to represent parsed code elements.

use crate::entity;
use crate::parser::*;

/// Represents the different types of nodes in the AST.
#[derive(Debug)]
pub enum NodeTypes<'a> {
    Enum(&'a entity::Enum),
    Function(&'a entity::Function),
    Struct(&'a entity::Struct),
}

/// Represents a node in the AST, containing a comment and a value.
#[derive(Debug)]
pub struct Node<'a> {
    comment: Option<&'a entity::DocComment>,
    value: Option<NodeTypes<'a>>,
}

/// Represents the entire Abstract Syntax Tree.
#[derive(Debug)]
pub struct AST<'a> {
    ast: Vec<Node<'a>>,
}

impl<'a> Node<'a> {
    /// Creates a new Node from a ParsedToken and an optional DocComment.
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

    /// Returns the comment associated with this node, if any.
    pub fn get_comment(&self) -> Option<&entity::DocComment> {
        self.comment
    }

    /// Returns the value of this node, if any.
    pub fn get_value(&self) -> &Option<NodeTypes> {
        &self.value
    }
}

impl<'a> AST<'a> {
    /// Builds an AST from a vector of ParsedTokens.
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

    /// Returns an iterator over the nodes in the AST.
    pub fn get_iter(&self) -> std::slice::Iter<'_, Node> {
        self.ast.iter()
    }
}
