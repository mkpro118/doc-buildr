use doc_buildr::entity::*;
use doc_buildr::parser::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_doc_comment() {
        let comment = r#"/**
         * This is a test function.
         * @param x The first parameter
         * @param y The second parameter
         * @return The sum of x and y
         */"#;

        let doc_comment = DocComment::parse(comment).unwrap();
        assert_eq!(doc_comment.comment.trim(), "This is a test function.");
        assert_eq!(doc_comment.params.len(), 2);
        assert_eq!(doc_comment.params[0].name, "x");
        assert_eq!(doc_comment.params[0].description, "The first parameter");
        assert!(doc_comment.retval.is_some());
        assert_eq!(
            doc_comment.retval.unwrap().description,
            "The sum of x and y"
        );
    }

    #[test]
    fn test_parse_function() {
        let function_str = "int add(int x, int y)";
        let function = Function::parse(function_str).unwrap();
        assert_eq!(function.name, "add");
        assert_eq!(function.return_type, "int");
        assert_eq!(function.params, vec!["int x", "int y"]);
    }

    #[test]
    fn test_parse_struct() {
        let struct_str = "struct Point { int x; int y; }";
        let struct_def = Struct::parse(struct_str).unwrap();
        assert_eq!(struct_def.name, "Point");
        assert_eq!(struct_def.members, vec!["int x", "int y"]);
    }

    #[test]
    fn test_parse_enum() {
        let enum_str = "enum Color { RED, GREEN, BLUE }";
        let enum_def = Enum::parse(enum_str).unwrap();
        assert_eq!(enum_def.name, "Color");
        assert_eq!(enum_def.variants, vec!["RED", "GREEN", "BLUE"]);
    }
}
