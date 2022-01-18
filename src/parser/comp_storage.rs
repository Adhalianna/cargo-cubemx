use std::collections::{HashMap, hash_map::Entry};
use patricia_tree::{PatriciaMap};

pub type Properties = PatriciaMap<String>;  
#[derive(Debug)]
pub struct Components{
    pub components: HashMap<String, Properties>
}

impl Components {
    pub fn new() -> Self {
        Self {
            components: HashMap::new()
        }
    }
    pub fn add_or_extend(&mut self, comp_name: &str, property: &str, value: String) {
        match self.components.entry(comp_name.to_string()) {
            Entry::Occupied(mut c) =>{
                (*c.get_mut()).insert(property, value);
            }
            Entry::Vacant(c) => {
                let mut p = Properties::new();
                p.insert(property, value);
                c.insert(p);
            }
        }
    }
}

