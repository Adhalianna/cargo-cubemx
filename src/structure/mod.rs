pub mod dma;
pub mod mcu;
pub mod signal;
pub mod rcc;

use enum_as_inner::EnumAsInner;
use std::collections::HashMap;

use crate::parser::comp_storage::Components;

use self::signal::Signal;


#[derive(Debug, EnumAsInner, Clone)]
#[non_exhaustive]
pub enum Component {
    MCU(mcu::Mcu),
    DMA(dma::Dma),
    SIGNAL(signal::Signal),
    PIN(signal::Pin),
    RCC(rcc::Rcc),
    Ignored,
}

pub fn structure_data(data: Components) -> HashMap<String, Component> {
    let mut all_components = HashMap::<String, Component>::new();
    for (component_name, properties) in data.components {
        match &component_name as &str {
            "Mcu" => {
                let mcu = mcu::create_mcu(properties)
                    .expect("An information about MCU family was missing in the configuration.");
                all_components.insert(component_name, Component::MCU(mcu));
            }
            "Dma" => {
                dma::create_dmas(properties)
                    .into_iter()
                    .map(|d| ((&d).dma_instance().to_owned(), Component::DMA(d)))
                    .for_each(|(k, v)| {
                        all_components.insert(k.to_string(), v);
                    });
            }
            "RCC" => {}
            "GPIO" => {}
            other @ _ => {
                match other.chars().next().unwrap() {
                    'P' => {
                        //PIN
                        let p = signal::create_pin(&component_name, properties);
                        all_components.insert(component_name, Component::PIN(p));
                    }
                    'U' => {
                        //USART
                        let u = signal::create_usart(&component_name, properties);
                        all_components.insert(component_name, Component::SIGNAL(Signal::USART(u)));
                    }
                    _ => {
                        panic!("An unexpected component {:?} met!", &component_name);
                        //TODO: Expand the message
                    }
                }
            }
        }
    }
    all_components
}
