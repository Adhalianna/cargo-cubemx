use std::path::Path;
mod parser;

fn main() {
    parser::parse(Path::new("src/test.ioc"));
}
