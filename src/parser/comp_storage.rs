//! A module for storing the data read by the parser in a "dirty" form.
//! It stores all the components (where, for example, all `Mcu` related settings
//! create a single component) in vectors organized with ``CompStorage`` structure.

use core::fmt;
use std::collections::{HashMap, hash_map::Entry};

pub struct CompStorage{
    pub components: HashMap<String, Vec<Property>>
}

impl CompStorage {
    pub fn new() -> Self {
        Self {
            components: HashMap::new()
        }
    }
    pub fn add_or_extend(&mut self, comp_name: &str, prop: Property) {
        match self.components.entry(comp_name.to_string()) {
            Entry::Occupied(mut c) =>{
                (*c.get_mut()).push(prop);
            }
            Entry::Vacant(c) => {
                c.insert(vec![prop]);
            }
        }
    }
}

impl fmt::Display for CompStorage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (k, v) in &self.components {
            write!(f, "{}: ", k)?;
            for p in v {
                write!(f, "{} ", p)?;
            }
        }
        Ok(())
    }
}

pub struct Property{
    pub elems: Vec<String>,
    pub value: String,
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "P(name: ")?;
        if self.elems.len() == 1 {
            write!(f, "{}", &self.elems[0])?;
        }
        else {
            let mut iter = self.elems.iter();
            write!(f, "{}", iter.next().unwrap())?;
            while let Some(e) = iter.next() {
                write!(f, ".{}", e)?;
            }
        }
        write!(f, ", value: {})", &self.value)?;
        Ok(())
    }
}