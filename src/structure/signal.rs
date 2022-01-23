use derive_builder::Builder;
use derive_getters::Getters;
use enum_as_inner::EnumAsInner;
use std::collections::HashMap;

#[derive(EnumAsInner, Debug, Clone)]
#[non_exhaustive]
pub enum Signal {
    USART(Usart),
    CAN,
}

#[derive(Debug, Builder, Getters, Clone)]
pub struct Usart {
    pub name: String,
    pub baudrate: u32,
}

#[derive(Builder, Debug, Getters, Clone)]
pub struct Pin {
    name: String,
    alias: Option<String>,
    signal: String,
}

pub fn create_pin(name: &str, data: HashMap<String, String>) -> Pin {
    let mut builder = PinBuilder::default();
    builder.alias(None); // TODO: Someday solve the problem of aliases
    for (property, value) in data {
        match &property as &str {
            "Signal" => {
                builder.signal(value);
            }
            _ => {}
        }
    }
    builder
        .name(name.to_string())
        .build()
        .expect("Failed to build a Pin wtih the provided information")
    //TODO: Expand the error message
}

pub fn create_usart(name: &str, data: HashMap<String, String>) -> Usart {
    let mut builder = UsartBuilder::default();
    for (property, value) in data {
        match &property as &str {
            "BaudRate" => {
                builder.baudrate((&value).parse::<u32>().unwrap());
            }
            _ => {}
        }
    }
    builder.name(name.to_string());
    builder.build().expect("Failed to build USART")
}
