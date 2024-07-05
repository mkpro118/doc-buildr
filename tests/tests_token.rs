use doc_buildr::token::Token;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input = r#"
        /**
         * This is a test function.
         * @param x The first parameter
         * @param y The second parameter
         * @return The sum of x and y
         */
        int add(int x, int y);
        "#;

        let tokens = Token::tokenize(input);
        assert_eq!(tokens.len(), 2);
        assert!(matches!(tokens[0].token, Token::DocComment));
        assert!(matches!(tokens[1].token, Token::Function));
    }
}
