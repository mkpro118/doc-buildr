//! # Tokenization Module
//!
//! This module provides a tokenization system for parsing specific structures in code,
//! particularly focused on C-style syntax elements. It offers functionality to identify
//! and extract doc comments, functions, structs, and enums from a given code string.

use regex::*;

/// Represents the types of tokens that can be identified.
#[derive(Debug)]
pub enum Token {
    /// Represents a documentation comment.
    DocComment,

    /// Represents a function declaration.
    Function,

    /// Represents a struct declaration.
    Struct,

    /// Represents a enum declaration.
    Enum,
}

/// A struct holding a token name and its corresponding regex pattern.
#[derive(Debug)]
struct TokenPatternPair(&'static str, &'static str);

/// Represents a matched token and its value.
#[derive(Debug)]
pub struct TokenValuePair {
    /// The type of the matched token.
    pub token: Token,

    /// The actual text content of the matched token.
    pub value: String,
}

impl TokenValuePair {
    /// Creates a `TokenValuePair` from a regex `Captures` object.
    pub fn from_capture(capture: &Captures) -> Self {
        if capture.name("DocComment").is_some() {
            Self {
                token: DocComment,
                value: String::from(&capture["DocComment"]),
            }
        } else if capture.name("Function").is_some() {
            Self {
                token: Function,
                value: String::from(&capture["Function"]),
            }
        } else if capture.name("Struct").is_some() {
            Self {
                token: Struct,
                value: String::from(&capture["Struct"]),
            }
        } else if capture.name("Enum").is_some() {
            Self {
                token: Enum,
                value: String::from(&capture["Enum"]),
            }
        } else {
            panic!("Expected named field!");
        }
    }
}

pub use Token::*;
impl<'a> Token {
    /// Returns a static array of `TokenPatternPair`s for all token types.
    fn get_pairs() -> &'static [TokenPatternPair] {
        static NAMES: [TokenPatternPair; 4] = [
            TokenPatternPair(DocComment.name(), DocComment.as_str()),
            TokenPatternPair(Function.name(), Function.as_str()),
            TokenPatternPair(Struct.name(), Struct.as_str()),
            TokenPatternPair(Enum.name(), Enum.as_str()),
        ];

        &NAMES
    }

    /// Returns the name of the token as a string.
    pub const fn name(&self) -> &'static str {
        match self {
            Token::DocComment => "DocComment",
            Token::Function => "Function",
            Token::Struct => "Struct",
            Token::Enum => "Enum",
        }
    }

    /// Returns the regex pattern for the token as a string.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Token::DocComment => r"/\*\*(.*?)\*/",
            Token::Function => r"\w+\s+\w+\s*\([^)]*\)\s*;",
            Token::Struct => r"(typedef)?struct\s+\w+\s*\{[^}]*\}\s*(\w+)?;",
            Token::Enum => r"(typedef)?enum\s+\w+\s*\{[^}]*\}\s*(\w+)?;",
        }
    }

    /// Constructs and returns a `Regex` object that can match all token types.
    pub fn get_regex() -> Regex {
        let pattern = Self::get_pairs()
            .iter()
            .map(|x| format!("(?<{}>{})", x.0, x.1))
            .collect::<Vec<_>>()
            .join("|");

        RegexBuilder::new(&pattern)
            .multi_line(true)
            .dot_matches_new_line(true)
            .build()
            .unwrap()
    }

    /// Tokenizes the given code string and returns a vector of `TokenValuePair`s.
    pub fn tokenize(code: &'a str) -> Vec<TokenValuePair> {
        Token::get_regex()
            .captures_iter(code)
            .map(|capture| TokenValuePair::from_capture(&capture))
            .collect::<Vec<TokenValuePair>>()
    }
}
