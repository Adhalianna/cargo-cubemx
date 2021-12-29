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

pub struct CompData{
    pub name: String,
    pub properties: Vec<Property>,
}

pub struct Property{
    pub elems: Vec<String>,
    pub value: String,
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(name: ")?;
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