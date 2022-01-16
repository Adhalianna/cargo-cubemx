pub struct Dma {
    pub dma_instace: String,
    pub stream_instance: String,
    pub signal: String,
    pub channel: String,
    pub priority: DmaPriority,
    pub fifo_mode: bool,
    pub mem_inc: bool,
    pub per_inc: bool,
}

impl Dma{
    pub fn new(dma: &str, stream: &str, signal: &str, channel: &str, priority: DmaPriority) -> Self {
        Self {
            dma_instace: dma.to_string(),
            stream_instance: stream.to_string(),
            signal: signal.to_string(),
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
