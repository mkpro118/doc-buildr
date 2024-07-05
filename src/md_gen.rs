use crate::ast::{AST, Node, NodeTypes};

fn generate_md(ast: AST) -> String {
    ast.get_iter()
       .map(Node::md_gen_visit)
       .collect::<Vec<_>>()
       .join("\n\n")
}

impl Node {
    fn md_gen_visit(&self) -> String {
        match self.get_value() {
            Some(node_type) => node_type.md_gen_visit(self.get_comment()),
            None => "".to_string(),
        }
    }
}

impl NodeTypes {
    fn md_gen_visit(&self, comment: &str) -> String {
        match self {
            NodeTypes::Enum(_) => self.md_gen_visit_enum(comment),
            NodeTypes::Function(_) => self.md_gen_visit_function(comment),
            NodeTypes::Struct(_) => self.md_gen_visit_struct(comment),
        }
    }

    fn md_gen_visit_enum(&self, comment: &str) -> String {
        let NodeTypes::Enum(node) = self else { panic!("Wrong type") };
        let mut md = String::new();
        md.push_str(format!("## Enum {}\n\n", node.name).as_str());
        md.push_str(format!("{}\n\n", comment).as_str());
        md.push_str("**Variants**:\n");

        for variant in &node.variants {
            md.push_str(format!("- {}\n", variant).as_str());
        }

        md
    }

    fn md_gen_visit_function(&self, comment: &str) -> String {
        unimplemented!()
    }

    fn md_gen_visit_struct(&self, comment: &str) -> String {
        unimplemented!()
    }
}
