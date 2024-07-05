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
            Some(node_type) => node_type.md_gen_visit(),
            None => "".to_string(),
        }
    }
}

impl NodeTypes {
    fn md_gen_visit(&self) -> String {
        match self {
            Enum => self.md_gen_visit_enum(),
            Function => self.md_gen_visit_function(),
            Struct => self.md_gen_visit_struct(),
        }
    }

    fn md_gen_visit_enum(&self) -> String;

    fn md_gen_visit_function(&self) -> String;

    fn md_gen_visit_struct(&self) -> String;
}
