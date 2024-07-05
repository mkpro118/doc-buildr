use doc_buildr::ast::*;
use doc_buildr::entity::*;
use doc_buildr::parser::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_ast() {
        let tokens = vec![
            ParsedToken::DocComment(DocComment {
                comment: "Test function".to_string(),
                params: vec![],
                retval: None,
            }),
            ParsedToken::Function(Function {
                name: "test".to_string(),
                return_type: "void".to_string(),
                params: vec![],
            }),
        ];

        let ast = AST::build_ast(&tokens);
        assert_eq!(ast.get_iter().count(), 1);
        let node = ast.get_iter().next().unwrap();
        assert!(node.get_comment().is_some());
        assert!(matches!(
            node.get_value().as_ref().unwrap(),
            NodeTypes::Function(_)
        ));
    }
}
