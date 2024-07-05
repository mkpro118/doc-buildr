use crate::ast::{Node, NodeTypes, AST};
use crate::entity;

pub fn generate_md(ast: &AST) -> String {
    ast.get_iter()
        .map(Node::md_gen_visit)
        .collect::<Vec<_>>()
        .join("\n\n")
}

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

fn md_escape(content: &str) -> String {
    content
        .split("\n")
        .map(replace_leading_whitespace)
        .collect::<Vec<String>>()
        .join("\n")
}

impl<'a> Node<'a> {
    fn md_gen_visit(&self) -> String {
        match self.get_value() {
            Some(node_type) => node_type.md_gen_visit(self.get_comment()),
            None => "".to_string(),
        }
    }
}

impl<'a> NodeTypes<'a> {
    fn md_gen_visit(&self, comment: Option<&'a entity::DocComment>) -> String {
        match self {
            NodeTypes::Enum(_) => self.md_gen_visit_enum(comment),
            NodeTypes::Function(_) => self.md_gen_visit_function(comment),
            NodeTypes::Struct(_) => self.md_gen_visit_struct(comment),
        }
    }

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
        md.push_str(format!("Returns `{}`: {}\n\n", node.return_type, ret_str).as_str());
        md.push_str("**Parameters**:\n");

        for param in &node.params {
            md.push_str(format!("- `{}`\n", param).as_str());
        }

        md
    }

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
