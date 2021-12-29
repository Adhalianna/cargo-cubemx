use std::path::Path;
mod parser;
mod structures;
mod codegen;

fn main() {
    parser::parse(Path::new("src/test.ioc")).unwrap();
}
