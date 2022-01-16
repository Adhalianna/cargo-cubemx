pub mod mcu;
pub mod dma;
pub mod signal;


use crate::parser::comp_storage::{CompStorage, Property};

/// Enlists all elements of CubeMX configuration supported by the compiler.
/// Components not supported at the moment by the compiler are enumerated as "Ignored".
#[non_exhaustive]
pub enum Component{
    MCU(mcu::Mcu),
    DMA(dma::Dma),
    USART(signal::Usart),
    PIN(signal::Pin),
    Ignored,
}

pub fn structure_data(data: CompStorage) -> Vec<Component>{
    let all_components = Vec::<Component>::new();
    for (k, v) in data.components {
        match &k as &str{
            "Dma" => { structure_dma(v); }
            "Mcu" => {}
            "RCC" => {}
            other @ _ => { if other.starts_with("P") && other.len() == 3 { // Pin

            }
            if other.starts_with("USART"){ // USART

            }
            }
        }
    }
    all_components
}

fn structure_dma(data: Vec<Property>) -> Vec<dma::Dma> {
    let dmas = Vec::<dma::Dma>::new();
    dmas
}