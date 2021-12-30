pub mod mcu;
pub mod dma;
pub mod signal;

use crate::parser::comp_storage::{CompStorage, Property};

/// Enlists all elements of CubeMX configuration supported by the compiler.
/// Components not supported at the moment by the compiler are enumerated as "Ignored".
#[non_exhaustive]
pub enum Component{
    MCU(mcu::Mcu),
    RCC,
    DMA(dma::Dma),
    USART(signal::Usart),
    PIN(signal::Pin),
    Ignored,
}

pub fn structure_data(data: CompStorage) -> Vec<Component>{
    let all_components = Vec::<Component>::new();
    for c in data.components {
        match &c.name as &str {
            "Dma" => {}
            "Mcu" => {}
            "RCC" => {}
            other @ _ => { if other.starts_with("P") && other.len() == 3 {

            }
            if other.starts_with("USART"){

            }
            }
        }
    }
    all_components
}

fn structure_dma(data: Vec<Property>) -> Vec<dma::Dma> {
    
}