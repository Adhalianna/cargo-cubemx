use std::path::Path;
mod parser;
mod structure;
mod codegen;

fn compile(from: &Path) {
    let component_store = parser::parse(from).unwrap();
}

fn main() {
    compile(Path::new("src/test.ioc"));
}
