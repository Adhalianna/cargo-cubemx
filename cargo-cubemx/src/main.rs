use std::io::Write;
use std::path::Path;
use std::env;

mod codegen;
mod parser;
mod structure;

fn compile(from: &Path, to: &Path) {
    let component_store = parser::parse(from).unwrap();
    let structured_components = structure::structure_data(component_store);
    let code = codegen::generate_code(&structured_components);
    // code = rustfmt_core::format_snippet(code, &rustfmt_core::Config::default()).unwrap();
    let mut file = std::fs::File::create(to).expect("create failed");
    file.write_all(&code.as_bytes())
        .expect("Writing compilation results to file failed.");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 4);
    compile(Path::new(&args[2]), Path::new(&args[3]));
}
