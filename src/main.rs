use std::fs;

mod ast;
mod entity;
mod md_gen;
mod parser;
mod token;

fn build_docs(input_file: &str, output_file: Option<&str>) -> Result<(), String> {
    let Ok(data) = fs::read_to_string(input_file) else {
        return Err(format!("File '{}' not found", &input_file));
    };

    let tokens = token::Token::tokenize(&data);
    let parsed = parser::parse_tokens(&tokens);
    let ast = ast::AST::build_ast(&parsed);
    let md = md_gen::generate_md(&ast);

    match output_file {
        Some(_file) => {}
        None => println!("{}", md),
    };

    Ok(())
}

fn main() {
    let filename = std::env::args().nth(1).expect("No arguments");
    match build_docs(&filename, None) {
        Ok(_) => println!("Done!"),
        Err(msg) => println!("Failed! {}", msg),
    }
}
