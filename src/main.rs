use std::io::Write;
use std::path::Path;

mod codegen;
mod parser;
mod structure;

fn compile(from: &Path) {
    let component_store = parser::parse(from).unwrap();
    let structured_components = structure::structure_data(component_store);
    let mut code = codegen::generate_code(&structured_components);
    // code = rustfmt_core::format_snippet(code, &rustfmt_core::Config::default()).unwrap();
    let mut file = std::fs::File::create("out.rs").expect("create failed");
    file.write_all(&code.as_bytes())
        .expect("writing ti file failed");
}

fn main() {
    compile(Path::new("src/test.ioc"));
}
