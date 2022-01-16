use std::path::Path;
mod parser;
mod structure;
mod codegen;

fn compile(from: &Path) {
    let component_store = parser::parse(from).unwrap();
    let structured_components = structure::structure_data(component_store);
}

fn main() {
    compile(Path::new("src/test.ioc"));
}
