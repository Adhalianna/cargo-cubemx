use std::{collections::HashMap, ops::Add, slice::SliceIndex};

use codegen::{Function, Scope};

use crate::structure::signal::{Pin, Signal};
use crate::structure::{
    dma::{Dma, DmaDirection, DmaPriority},
    Component,
};

pub fn generate_code(data: &HashMap<String, Component>, scope: &mut Scope) {
    for (k, v) in data {
        match v {
            Component::DMA(d) => {
                scope.push_fn(dma_func(d));
            }
            Component::PIN(p) => {
                let signal = data
                    .iter()
                    .find(|s| p.signal().starts_with(s.0))
                    .map(|s| s.1)
                    .expect(
                        "The signal assigned to the pin was not described in the configuration.",
                    )
                    .as_signal()
                    .unwrap();
                scope.push_fn(signal_func(p, signal));
            }
            _ => {}
        }
    }
}

fn dma_func(d: &Dma) -> Function {
    let mut func = codegen::Function::new(&"init_".to_string().add(d.dma_instance()));
    func.doc("TODO")
        .arg("per", "dma::traits::DMASet + dma::traits::PeriAddress")
        .arg("buf", "embedded_dma::StaticWriteBuffer")
        .ret("dma::Transfer")
        .vis("pub");

    func.line("let dp = pac::Peripherals::take().unwrap();");

    let which_dma = d.dma_instance().get(0..4).unwrap();
    func.line(format!("let dma = dp.{};", which_dma));

    let name_last_i = d.dma_instance().len() - 1;
    let which_stream = d
        .dma_instance()
        .get(name_last_i..(name_last_i + 1))
        .unwrap();
    func.line(format!(
        "let stream = dma::StreamsTuple::new(dma).{};",
        which_stream
    ));

    let priority_str = match d.priority() {
        DmaPriority::Low => "Low",
        DmaPriority::Medium => "Medium",
        DmaPriority::High => "High",
        DmaPriority::VeryHigh => "VeryHigh",
    };
    func.line("let config = dma::config::DmaConfig::default()")
        .line(format!(
            ".priority(dma::config::Priority::{})",
            priority_str
        ));

    if *d.mem_inc() {
        func.line(".memory_increment(true)");
    }

    if *d.per_inc() {
        func.line(".peripheral_increment(true)");
    }

    if *d.fifo_mode() {
        func.line(".fifo_enable(true)");
    }

    func.line(";");

    let direction = match d.direction() {
        DmaDirection::PeripheralToMemory => {
            func.line("dma::Transfer::init_peripheral_to_memory(");
        }
        DmaDirection::MemoryToPeripheral => {
            func.line("dma::Transfer::init_memory_to_peripheral(");
        }
        DmaDirection::MemoryToMemory => {
            func.line("dma::Transfer::init_memory_to_memory(");
        }
    };

    func.line("stream,")
        .line("per,")
        .line("buf,")
        .line("None,") //TODO: Add the option to use a double buffer
        .line("config")
        .line(")");

    func
}

fn signal_func(p: &Pin, s: &Signal) -> Function {
    let mut func = codegen::Function::new(&"init_".to_string().add(p.signal()));
    match s {
        Signal::USART(u) => {
            func.arg("cloks", "rcc::Clocks");
            let rx_tx;
            if p.signal().ends_with("_RX") {
                func.ret("serial::Rx");
                rx_tx = "rx"
            } else {
                func.ret("serial::Tx");
                rx_tx = "tx"
            }
            func.line("let dp = pac::Peripheral::take().unwrap();");
            unsafe {
                let gpio_char: &str = p.name().get_unchecked(1..2);
                func.line(format!(
                    "let mut gpio = dp.GPIO{}.split();",
                    gpio_char
                ));
            }
            let pin_var = p.name().to_ascii_lowercase();
            func.line(format!(
                "let {} = gpio.{}.into_alternate();",
                pin_var, pin_var
            ))
            .line("let config = serial::config::default()")
            .line(format!(".baudrate({})", u.baudrate()))
            .line(";");

            func.line(format!("serial::Serial::{}(", rx_tx))
                .line(format!("dp.{},", u.name()))
                .line(format!("{},", pin_var))
                .line("config,")
                .line("clocks")
                .line(")");

            func.doc("TODO");
        }
        _ => {}
    }
    func
}
