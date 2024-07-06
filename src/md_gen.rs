//! # Markdown Generation Module
//!
//! This module is responsible for generating markdown documentation
//! from the Abstract Syntax Tree (AST) created by parsing the source code.

use crate::ast::{Node, NodeTypes, AST};
use crate::entity;

/// Generates markdown documentation from an AST.
///
/// # Arguments
///
/// * `ast` - A reference to the AST to generate documentation from.
///
/// # Returns
///
/// A `String` containing the generated markdown documentation.
pub fn generate_md(ast: &AST) -> String {
    ast.get_iter()
        .map(Node::md_gen_visit)
        .collect::<Vec<_>>()
        .join("\n\n")
}

/// Replaces leading whitespace with non-breaking spaces for markdown formatting.
fn replace_leading_whitespace(s: &str) -> String {
    let (leading_whitespace, rest) = s.split_at(
        s.chars()
            .position(|c| !c.is_whitespace())
            .unwrap_or(s.len()),
    );

    let nbsp_prefix = leading_whitespace
        .chars()
        .skip(1)
        .map(|_| "&nbsp;")
        .collect::<String>();

    nbsp_prefix + rest
}

/// Escapes special characters in the content for proper markdown rendering.
fn md_escape(content: &str) -> String {
    content
        .split("\n")
        .map(replace_leading_whitespace)
        .collect::<Vec<String>>()
        .join("\n")
}

impl<'a> Node<'a> {
    /// Generates markdown for this node.
    fn md_gen_visit(&self) -> String {
        match self.get_value() {
            Some(node_type) => node_type.md_gen_visit(self.get_comment()),
            None => "".to_string(),
        }
    }
}

impl<'a> NodeTypes<'a> {
    /// Generates markdown for this node type.
    fn md_gen_visit(&self, comment: Option<&'a entity::DocComment>) -> String {
        match self {
            NodeTypes::Enum(_) => self.md_gen_visit_enum(comment),
            NodeTypes::Function(_) => self.md_gen_visit_function(comment),
            NodeTypes::Struct(_) => self.md_gen_visit_struct(comment),
        }
    }

    /// Generates markdown for an enum.
    fn md_gen_visit_enum(&self, comment: Option<&'a entity::DocComment>) -> String {
        let comment_str: &str;

        match comment {
            Some(c) => comment_str = c.comment.as_str(),
            None => comment_str = "No documentation available",
        }

        let NodeTypes::Enum(node) = self else { panic!("Wrong type") };
        let mut md = String::new();
        md.push_str(format!("## Enum `{}`\n\n", node.name).as_str());
        md.push_str(format!("{}\n\n", md_escape(comment_str)).as_str());
        md.push_str("**Variants**:\n");

        for variant in &node.variants {
            md.push_str(format!("- `{}`\n", variant).as_str());
        }

        md
    }

    /// Generates markdown for a function.
    fn md_gen_visit_function(&self, comment: Option<&'a entity::DocComment>) -> String {
        let mut ret_str: &str = "No description";
        let mut comment_str: &str = "No documentation available";

        if let Some(c) = comment {
            comment_str = c.comment.as_str();

            if let Some(r) = &c.retval {
                ret_str = r.description.as_str();
            };
        }

        let NodeTypes::Function(node) = self else { panic!("Wrong type") };
        let mut md = String::new();
        md.push_str(format!("## Function `{}`\n\n", node.name).as_str());
        md.push_str(
            format!(
                "```c\n{} {}({})\n```\n\n",
                node.return_type,
                node.name,
                node.params.join(", ")
            )
            .as_str(),
        );
        md.push_str(format!("{}\n\n", md_escape(comment_str)).as_str());

        if node.return_type != "void" {
            md.push_str(
                format!("**Returns**:\n\n`{}`: {}\n\n", node.return_type, ret_str).as_str(),
            );
        }

        md.push_str("**Parameters**:\n");

        if let Some(c) = comment {
            node.params
                .iter()
                .map(|x| x.split(' ').last().unwrap())
                .fold((), |_, name| {
                    let desc = match c.params.iter().find(|p| p.name == name) {
                        Some(p) => p.description.as_str(),
                        None => "No description",
                    };

                    md.push_str(format!("- `{}`: {}\n", name, desc).as_str())
                })
        } else {
            node.params
                .iter()
                .map(|param| param.split(' ').last().unwrap())
                .fold((), |_, name| {
                    md.push_str(format!("- `{}`\n", name).as_str())
                })
        }

        md
    }

    /// Generates markdown for a struct.
    fn md_gen_visit_struct(&self, comment: Option<&'a entity::DocComment>) -> String {
        let comment_str: &str;

        match comment {
            Some(c) => comment_str = c.comment.as_str(),
            None => comment_str = "No documentation available",
        }

        let NodeTypes::Struct(node) = self else { panic!("Wrong type") };
        let mut md = String::new();
        md.push_str(format!("## Struct `{}`\n\n", node.name).as_str());
        md.push_str(format!("{}\n\n", md_escape(comment_str)).as_str());
        md.push_str("**Members**:\n");

        for member in &node.members {
            md.push_str(format!("- `{}`\n", member).as_str());
        }

        md
    }
}
