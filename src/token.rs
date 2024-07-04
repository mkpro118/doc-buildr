use regex::{*};

#[derive(Debug)]
pub enum Token {
    DocComment,
    Function,
    Struct,
    Enum,
}

#[derive(Debug)]
struct TokenPatternPair(&'static str, &'static str);

#[derive(Debug)]
pub struct TokenValuePair{
    token: Token,
    value: String,
}

impl TokenValuePair {
    pub fn from_capture(capture: &Captures) -> Self {
        if capture.name("DocComment").is_some() {
            Self {
                token: DocComment,
                value: String::from(&capture["DocComment"])
            }
        } else if capture.name("Function").is_some() {
            Self {
                token: Function,
                value: String::from(&capture["Function"])
            }
        } else if capture.name("Struct").is_some() {
            Self {
                token: Struct,
                value: String::from(&capture["Struct"])
            }
        } else if capture.name("Enum").is_some() {
            Self {
                token: Enum,
                value: String::from(&capture["Enum"])
            }
        } else {
            panic!("Expected named field!");
        }
    }
}


use Token::{*};
impl<'a> Token {
    fn get_pairs() -> &'static [TokenPatternPair] {
        static NAMES: [TokenPatternPair; 4] = [
            TokenPatternPair(DocComment.name(), DocComment.as_str()),
            TokenPatternPair(Function.name(), Function.as_str()),
            TokenPatternPair(Struct.name(), Struct.as_str()),
            TokenPatternPair(Enum.name(), Enum.as_str()),
        ];

        &NAMES
    }

    pub const fn name(&self) -> &'static str {
        match self {
            Token::DocComment => "DocComment",
            Token::Function => "Function",
            Token::Struct => "Struct",
            Token::Enum => "Enum",
        }
    }

    pub fn from_name(name: &str) -> Self {
        match name {
            "DocComment" => Token::DocComment,
            "Function" => Token::Function,
            "Struct" => Token::Struct,
            "Enum" => Token::Enum,
            _ => panic!("Unknown token name"),
        }
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            Token::DocComment => r"/\*\*(.*?)\*/",
            Token::Function => r"\w+\s+\w+\s*\([^)]*\)\s*;",
            Token::Struct => r"struct\s+\w+\s*\{[^}]*\}\s*;",
            Token::Enum => r"enum\s+\w+\s*\{[^}]*\}\s*;",
        }
    }

    pub fn get_regex() -> Regex {
        let pairs = Self::get_pairs();
        let pairs = pairs.iter().map(|x| format!("(?<{}>{})", x.0, x.1));

        let pattern = pairs.collect::<Vec<_>>().join("|");

        dbg!(&pattern);
        let regex = RegexBuilder::new(&pattern)
            .multi_line(true)
            .dot_matches_new_line(true)
            .build()
            .unwrap();
        dbg!(&regex);

        regex
    }

    pub fn tokenize(code: &'a str) -> Vec<TokenValuePair> {
        let regex = Token::get_regex();

        let mut tokens: Vec<TokenValuePair> = vec![];
        for capture in regex.captures_iter(code) {
            tokens.push(TokenValuePair::from_capture(&capture));
        }

        // vec![]
        tokens
    }
}
