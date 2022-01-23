use pest::{error::Error as PestError, iterators::Pair, Parser};
use pest_derive::*;
use std::{fs::read_to_string, path::Path};

pub mod comp_storage;
use self::comp_storage::Components;

/// Parser generated from Pest's PEG.
/// More information can be found in Pest's docs
#[derive(Parser)]
#[grammar = "parser/ioc_grammar.pest"]
struct IOCParser;

//TODO: remove the unwrap at file reading and change the return type
pub fn parse(path: &Path) -> Result<Components, PestError<Rule>> {
    let file = read_to_string(path).unwrap();
    parse_string(file.as_str())
}

pub fn parse_string(input: &str) -> Result<Components, PestError<Rule>> {
    let pairs = IOCParser::parse(Rule::file, input)?;
    let mut store = Components::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::assignment => {
                let (name, property, value) = decompose_assignment(pair);
                store.add_or_extend(name, property, value);
            }
            _ => {}
        }
    }
    Ok(store)
}

fn decompose_assignment(assignment: Pair<Rule>) -> (String, String, String) {
    // Decompose parsed data
    let mut assignment = assignment.into_inner();
    let mut component_elems = assignment.nth(0).unwrap().into_inner();
    // Save property value
    let value = assignment.next().unwrap().as_str().to_string();
    // Get component name
    let comp_name = component_elems.next().unwrap().as_str().to_string();
    // Decompose pair into a property string
    let property = component_elems.next();
    let property_string: String;
    if let Some(property) = property {
        property_string = property.as_str().strip_prefix(".").unwrap().to_string();
    } else {
        property_string = "".to_string();
    }
    (comp_name, property_string, value)
}
