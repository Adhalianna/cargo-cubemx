use std::{collections::HashMap, ops::Add};

use codegen::Scope;

use crate::structure::{mcu::McuFamily, Component};

mod f4;

pub fn generate_code(data: &HashMap<String, Component>) -> String {
    let mut scope = Scope::new();
    let mcu = data.get("Mcu").unwrap().as_mcu().unwrap();
    let underlying_hal = mcu.family_string().to_owned().add("xx_hal");
    scope.import(&underlying_hal, "*");
    match mcu.family() {
        McuFamily::F4 => f4::generate_code(data, &mut scope),
        _ => {
            panic!("Chosen Mcu family is not supported")
        }
    }
    scope.to_string()
}
