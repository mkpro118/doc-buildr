use doc_buildr::ast::*;
use doc_buildr::entity::*;
use doc_buildr::md_gen::generate_md;
use doc_buildr::parser::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_md() {
        let tokens = vec![
            ParsedToken::DocComment(DocComment {
                comment: "Test function".to_string(),
                params: vec![Param {
                    name: "x".to_string(),
                    description: "Input parameter".to_string(),
                }],
                retval: Some(Return {
                    description: "Output value".to_string(),
                }),
            }),
            ParsedToken::Function(Function {
                name: "test".to_string(),
                return_type: "int".to_string(),
                params: vec!["int x".to_string()],
            }),
        ];

        let ast = AST::build_ast(&tokens);
        let md = generate_md(&ast);

        assert!(md.contains("## Function `test`"));
        assert!(md.contains("Test function"));
        assert!(md.contains("**Parameters**:"));
        assert!(md.contains("- `x`: Input parameter"));
        assert!(md.contains("**Returns**:"));
        assert!(md.contains("`int`: Output value"));
    }
}
