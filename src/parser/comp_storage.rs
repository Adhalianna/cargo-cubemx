use std::collections::{hash_map::Entry, HashMap};

#[derive(Debug)]
pub struct Components {
    pub components: HashMap<String, HashMap<String, String>>,
}

impl Components {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }
    pub fn add_or_extend(&mut self, comp_name: String, property: String, value: String) {
        match self.components.entry(comp_name.to_string()) {
            Entry::Occupied(mut c) => {
                (*c.get_mut()).insert(property, value);
            }
            Entry::Vacant(c) => {
                let mut m = HashMap::<String, String>::new();
                m.insert(property, value);
                c.insert(m);
            }
        }
    }
}
