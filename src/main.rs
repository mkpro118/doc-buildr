use std::fs;
use std::io::stdout;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

mod ast;
mod entity;
mod md_gen;
mod parser;
mod token;

fn build_docs(input_file: &str) -> Result<String, String> {
    let Ok(data) = fs::read_to_string(input_file) else {
        return Err(format!("File '{}' not found", &input_file));
    };

    let filename = Path::new(input_file).file_stem().unwrap().to_str().unwrap();

    let tokens = token::Token::tokenize(&data);
    let parsed = parser::parse_tokens(&tokens);
    let ast = ast::AST::build_ast(&parsed);
    let md = md_gen::generate_md(&ast);

    let mut docs = String::from(format!("# Module {}\n\n", filename));
    docs.push_str(&md);

    Ok(docs)
}

fn main() {
    enum ArgType {
        InputFile,
        OutputFile,
    }
    use ArgType::*;

    let (inputs, output, _) = std::env::args().skip(1).fold(
        (None::<Vec<String>>, None::<String>, InputFile),
        |(mut inputs, mut output, mut mode), arg| {
            match arg.as_str() {
                "-o" | "--output" => mode = OutputFile,
                _ => match mode {
                    InputFile => {
                        if let Some(ref mut files) = inputs {
                            files.push(arg);
                        } else {
                            inputs = Some(vec![arg]);
                        }
                    }
                    OutputFile => {
                        output = Some(arg);
                        mode = InputFile;
                    }
                },
            };

            (inputs, output, mode)
        },
    );

    let Some(inputs) = inputs else {
        println!("No input files!");
        return;
    };

    let output_file: &str;

    let mut writer: Box<dyn Write> = Box::new(BufWriter::new(match output {
        Some(ref x) => {
            output_file = x.as_str();
            Box::new(fs::File::create(&Path::new(x)).unwrap()) as Box<dyn Write>
        }
        None => {
            output_file = "stdout";
            Box::new(stdout()) as Box<dyn Write>
        }
    }));

    inputs.iter().fold((), |_, file| {
        match build_docs(file) {
            Ok(docs) => {
                writer
                    .write(docs.as_bytes())
                    .expect(format!("Failed to write to file {}", output_file).as_str());
            }
            Err(msg) => {
                eprintln!("{}", msg);
            }
        };
    })
}
