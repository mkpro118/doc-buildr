use std::fs;

mod entity;
mod token;
mod parser;
mod ast;
mod md_gen;

fn build_docs(
    input_file: &str,
    output_file: Option<&str>
) -> Result<bool, &'static str> {

}

fn main() {
    let filename = "";
    let data = fs::read_to_string(filename).expect("File not found!");
    let tokens = token::Token::tokenize(&data);

    let parsed = parser::parse_tokens(&tokens);
    let ast = ast::AST::build_ast(&parsed);
    let md = md_gen::generate_md(&ast);

    println!("{}", md);
}
