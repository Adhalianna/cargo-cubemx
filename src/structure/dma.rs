use std::{collections::HashMap, ops::Add};

use derive_builder::Builder;
use derive_getters::Getters;

#[derive(Builder, Debug, Getters, Clone)]
pub struct Dma {
    dma_instance: String,
    signal: String,
    channel: String,
    priority: DmaPriority,
    direction: DmaDirection,
    fifo_mode: bool,
    mem_inc: bool,
    per_inc: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum DmaPriority {
    Low,
    Medium,
    High,
    VeryHigh,
}

impl Default for DmaPriority {
    fn default() -> Self {
        Self::Low
    }
}

#[derive(Clone, Copy, Debug)]
pub enum DmaDirection {
    MemoryToMemory,
    MemoryToPeripheral,
    PeripheralToMemory,
}

pub fn create_dmas(data: HashMap<String, String>) -> Vec<Dma> {
    let mut builders = HashMap::<String, DmaBuilder>::new();
    for (line, value) in data {
        let split_line = split_dma_property(line);
        let dma_identifier = split_line[0].to_owned().add(&split_line[1]);
        if let Some(builder) = builders.get_mut(&dma_identifier) {
            set_property(builder, &split_line[2], value);
        } else {
            let mut builder: DmaBuilder = DmaBuilder::default();
            builder
                .signal(split_line[0].to_owned())
                .channel(split_line[1].to_owned());
            set_property(&mut builder, &split_line[2], value);
            builders.insert(dma_identifier, builder);
        }
    }
    let dmas: Vec<Dma> = builders.into_iter().map(|a| a.1.build().unwrap()).collect();
    dmas
}

fn split_dma_property(prop: String) -> Vec<String> {
    prop.split('.')
        .map(|a| a.to_owned())
        .collect::<Vec<String>>()
}

fn set_property(builder: &mut DmaBuilder, prop: &str, value: String) {
    match prop {
        "Instance" => {
            builder.dma_instance(value);
        }
        "Priority" => match &value as &str {
            "DMA_PRIORITY_LOW" => {
                builder.priority(DmaPriority::Low);
            }
            "DMA_PRIORITY_MEDIUM" => {
                builder.priority(DmaPriority::Medium);
            }
            "DMA_PRIORITY_HIGH" => {
                builder.priority(DmaPriority::High);
            }
            "DMA_PRIORITY_VERY_HIGH" => {
                builder.priority(DmaPriority::VeryHigh);
            }
            s @ _ => {
                panic!(
                    "Failed to parse priority setting on a DMA instance. Provided value: {}",
                    s
                );
            }
        },
        "Direction" => {
            match &value as &str {
                "DMA_PERIPH_TO_MEMORY" => {
                    builder.direction(DmaDirection::PeripheralToMemory);
                }
                "DMA_MEMORY_TO_PERIPH" => {
                    builder.direction(DmaDirection::MemoryToPeripheral);
                }
                "DMA_MEMORY_TO_MEMORY" => {
                    builder.direction(DmaDirection::MemoryToMemory);
                }
                _s @ _ => panic!(), //TODO: add message
            }
        }
        "MemInc" => {
            match &value as &str {
                "DMA_MINC_ENABLE" => {
                    builder.mem_inc(true);
                }
                "DMA_MINC_DISABLE" => {
                    builder.mem_inc(false);
                }
                _s @ _ => panic!(), //TODO: add message
            }
        }
        "PeriphInc" => {
            match &value as &str {
                "DMA_PINC_ENABLE" => {
                    builder.per_inc(true);
                }
                "DMA_PINC_DISABLE" => {
                    builder.per_inc(false);
                }
                _s @ _ => panic!(), //TODO: add message
            }
        }
        "FIFOMode" => {
            match &value as &str {
                "DMA_FIFOMODE_ENABLE" => {
                    builder.fifo_mode(true);
                }
                "DMA_FIFOMODE_DISABLE" => {
                    builder.fifo_mode(false);
                }
                _s @ _ => panic!(), //TODO: add message
            }
        }
        "RequestParameters" => { /* Ignore */ }
        s @ _ => {
            println!(
                "Reached an unimplemented property of DMA {:?} with value {:?}",
                s, &value
            );
            //TODO: Add other possible properties and/or panic condition
        }
    }
}
