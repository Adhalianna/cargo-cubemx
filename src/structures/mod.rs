/// Enlists all elements of CubeMX configuration supported by the compiler.
/// Components not supported at the moment by the compiler are enumerated as "Ignored".
#[non_exhaustive]
pub enum Components{
    MCU,
    RCC,
    DMA,
    USART,
    PIN(Pin),
    Ignored,
}

pub struct Mcu {
    pub family: McuFamily
    //TODO?
}

/// Only families supported by the compiler are enlisted, other families fall into `Unsupported` category.
#[non_exhaustive]
pub enum McuFamily{
    F4,
    Unsupported,
}

pub struct Dma {
    pub dma_instace: String,
    pub stream_instance: String,
    pub signal: Signal,
    pub channel: String,
    pub priority: DmaPriority,
    pub fifo_mode: bool,
    pub mem_inc: bool,
    pub per_inc: bool,
}

#[non_exhaustive]
pub enum Signal{
    USART(Usart),
    CAN
}

impl Dma{
    pub fn new(dma: &str, stream: &str, signal: Signal, channel: &str, priority: DmaPriority) -> Self {
        Self {
            dma_instace: dma.to_string(),
            stream_instance: stream.to_string(),
            signal,
            channel: channel.to_string(),
            priority,
            fifo_mode: false,
            mem_inc: false,
            per_inc: false,
        }
    }
}

pub enum DmaPriority {
    Low,
    Medium,
    High,
    VeryHigh,
}

pub struct Pin {
    pub name: String,
    pub alias: Option<String>,
    pub signal: Signal,
}

impl Pin{
    pub fn new(name: &str, alias: Option<&str>, signal: Signal) -> Self {
        Self {
            name: name.to_string(),
            alias:
                if let Some(a) = alias { Some(a.to_string()) }
                else { None },
            signal,
        }
    }
}

pub struct Usart {
    pub receiver: bool,
    pub name: String,
    pub baudrate: u32,
}