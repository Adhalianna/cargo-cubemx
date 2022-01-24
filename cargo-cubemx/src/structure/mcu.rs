use std::collections::HashMap;

use derive_getters::Getters;

#[derive(Debug, Getters, Clone)]
pub struct Mcu {
    family: McuFamily,
    family_string: String,
}

impl Mcu {
    pub fn new(family: McuFamily, family_string: String) -> Self {
        Mcu {
            family,
            family_string,
        }
    }
}

/// Only families supported by the compiler are enlisted, other families fall into `Unsupported` category.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum McuFamily {
    F4,
    Unsupported,
}

pub fn create_mcu(data: HashMap<String, String>) -> Option<Mcu> {
    let mut mcu: Option<Mcu> = None;
    for (property, value) in data {
        match &property as &str {
            "Family" => {
                let family_enum = match &value as &str {
                    "STM32F4" => McuFamily::F4,
                    _ => McuFamily::Unsupported,
                };
                mcu = Some(Mcu::new(family_enum, value.to_ascii_lowercase()));
            }
            _s @ _ => {
                //TODO: might include something more someday
            }
        }
    }
    mcu
}
