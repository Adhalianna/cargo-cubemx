//! A module for storing the data read by the parser in a "dirty" form.
//! It stores all the components (where, for example, all `Mcu` related settings
//! create a single component) in vectors organized with ``CompStorage`` structure.

use core::fmt;

pub struct CompStorage{
    pub components: Vec<CompData>
}

impl CompStorage {
    pub fn new() -> Self {
        Self {
            components: Vec::<CompData>::new()
        }
    }
    fn find_component(&mut self, comp_name: &str) -> Option<&mut CompData> {
        for c in &mut self.components {
            if c.name.eq(comp_name) { return Some(c); }
        }
        None
    }
    pub fn add_or_extend(&mut self, comp_name: &str, prop: Property) {
        let found_duplicate = self.find_component(comp_name);
        match found_duplicate {
            Some(c) =>{
                c.properties.push(prop);
            }
            None => {
                self.components.push(CompData {
                    name: comp_name.to_string(),
                    properties: vec![prop],
                })
            }
        }
    }
}

impl fmt::Display for CompStorage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for e in &self.components {
            write!(f, "{} ", e)?;
        }
        Ok(())
    }
}

pub struct CompData{
    pub name: String,
    pub properties: Vec<Property>,
}

impl fmt::Display for CompData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: ", self.name)?;
        for e in &self.properties {
            write!(f, "{} ", e)?;
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