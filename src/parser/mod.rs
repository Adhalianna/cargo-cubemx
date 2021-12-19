use std::{fs::read_to_string, path::Path};
use pest::Parser;
use pest_derive::*;

#[derive(Parser)]
#[grammar = "parser/ioc_grammar.pest"]
struct IOCParser;

pub fn parse(path: &Path) {
    let file = read_to_string(path).unwrap();
    match IOCParser::parse(Rule::file, &file) {
        Ok(x) => println!("{}", x),
        Err(_err) => panic!()
    }
}