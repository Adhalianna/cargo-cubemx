use std::{fs::read_to_string, path::Path};
use pest::{Parser, error::Error as PestError, iterators::Pair};
use pest_derive::*;

mod comp_storage;
use self::comp_storage::{CompStorage, Property};

#[derive(Parser)]
#[grammar = "parser/ioc_grammar.pest"]
struct IOCParser;

//TODO: change return to dynamic error or restructure the program - io errors might
//occur while reading a file.
pub fn parse(path: &Path) -> Result<CompStorage, PestError<Rule>> {
    let file = read_to_string(path).unwrap();
    parse_string(file.as_str())
}

pub fn parse_string(input: &str) -> Result<CompStorage, PestError<Rule>> {
    let pairs =  IOCParser::parse(Rule::file, input)?;
    let mut store = CompStorage::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::assignment => {
                let (name, prop) = parse_assignment_to_data(pair);
                store.add_or_extend(name.as_str(), prop.unwrap());
            }
            _ => {}
        }
    }
    Ok(store)
}

fn parse_assignment_to_data(assignment: Pair<Rule>) -> (String, Option<Property>) {
    let mut assignment = assignment.into_inner();
    let mut component_elems = assignment.nth(0).unwrap().into_inner();
    let value = assignment.next().unwrap();

    let comp_name = component_elems.next().unwrap().as_str().to_string();

    let property = component_elems.next();
    let mut prop_elems = Vec::<String>::new();
    if let Some(prop) = property {
        let props = prop.into_inner();
        for p in props {
            if p.as_rule() == Rule::prop_elem {
                let elem = p.as_str().to_string();
                prop_elems.push(elem);
            }
        }
    };

    let prop = Property {
        elems: prop_elems,
        value: value.as_str().to_string(),
    };

    println!("c: {}, p: {}", comp_name, prop);
    (comp_name, Some(prop))
}