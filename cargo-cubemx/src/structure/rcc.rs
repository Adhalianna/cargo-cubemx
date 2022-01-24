use derive_builder::Builder;
use derive_getters::Getters;
use std::collections::HashMap;

#[derive(Getters, Debug, Clone, Builder)]
pub struct Rcc {
    saia: u32,
    saib: u32,
    hse: Option<u32>,
    hclk: u32,
    sys: u32,
}

pub fn create_clock(data: HashMap<String, String>) -> Rcc {
    let builder = RccBuilder::default();
    for (k, v) in data {
        match &k as &str {
            "SAIAFreq_Value" => {}
            "SAIBFreq_Value" => {}
            "HSE_VALUE" => {}
            "AHBFreq_Value" => {}
            "SYSCLKFreq_VALUE" => {}
            "PLLCLKFreq_Value" => {}
            "I2SClocksFreq_Value" => {}
            _ => {}
        }
    }
    builder.build().unwrap()
}
