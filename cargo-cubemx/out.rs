use stm32f4xx_hal::*;

/// TODO
pub fn init_DMA1_Stream6(per: dma::traits::DMASet + dma::traits::PeriAddress, buf: embedded_dma::StaticWriteBuffer) -> dma::Transfer {
    let dp = pac::Peripherals::take().unwrap();
    let dma = dp.DMA1;
    let stream = dma::StreamsTuple::new(dma).6;
    let config = dma::config::DmaConfig::default()
    .priority(dma::config::Priority::Low)
    .memory_increment(true)
    ;
    dma::Transfer::init_peripheral_to_memory(
    stream,
    per,
    buf,
    None,
    config
    )
}

/// TODO
pub fn init_DMA2_Stream4(per: dma::traits::DMASet + dma::traits::PeriAddress, buf: embedded_dma::StaticWriteBuffer) -> dma::Transfer {
    let dp = pac::Peripherals::take().unwrap();
    let dma = dp.DMA2;
    let stream = dma::StreamsTuple::new(dma).4;
    let config = dma::config::DmaConfig::default()
    .priority(dma::config::Priority::Low)
    .memory_increment(true)
    ;
    dma::Transfer::init_memory_to_peripheral(
    stream,
    per,
    buf,
    None,
    config
    )
}

/// TODO
pub fn init_DMA1_Stream1(per: dma::traits::DMASet + dma::traits::PeriAddress, buf: embedded_dma::StaticWriteBuffer) -> dma::Transfer {
    let dp = pac::Peripherals::take().unwrap();
    let dma = dp.DMA1;
    let stream = dma::StreamsTuple::new(dma).1;
    let config = dma::config::DmaConfig::default()
    .priority(dma::config::Priority::Low)
    .memory_increment(true)
    ;
    dma::Transfer::init_memory_to_peripheral(
    stream,
    per,
    buf,
    None,
    config
    )
}

/// TODO
fn init_UART8_RX(cloks: rcc::Clocks) -> serial::Rx {
    let dp = pac::Peripheral::take().unwrap();
    let mut gpio = dp.GPIOE.split();
    let pe0 = gpio.pe0.into_alternate();
    let config = serial::config::default()
    .baudrate(1000000)
    ;
    serial::Serial::rx(
    dp.UART8,
    pe0,
    config,
    clocks
    )
}

/// TODO
pub fn init_DMA1_Stream3(per: dma::traits::DMASet + dma::traits::PeriAddress, buf: embedded_dma::StaticWriteBuffer) -> dma::Transfer {
    let dp = pac::Peripherals::take().unwrap();
    let dma = dp.DMA1;
    let stream = dma::StreamsTuple::new(dma).3;
    let config = dma::config::DmaConfig::default()
    .priority(dma::config::Priority::Low)
    .memory_increment(true)
    ;
    dma::Transfer::init_peripheral_to_memory(
    stream,
    per,
    buf,
    None,
    config
    )
}

/// TODO
pub fn init_DMA2_Stream0(per: dma::traits::DMASet + dma::traits::PeriAddress, buf: embedded_dma::StaticWriteBuffer) -> dma::Transfer {
    let dp = pac::Peripherals::take().unwrap();
    let dma = dp.DMA2;
    let stream = dma::StreamsTuple::new(dma).0;
    let config = dma::config::DmaConfig::default()
    .priority(dma::config::Priority::Low)
    .memory_increment(true)
    ;
    dma::Transfer::init_peripheral_to_memory(
    stream,
    per,
    buf,
    None,
    config
    )
}

/// TODO
fn init_UART7_RX(cloks: rcc::Clocks) -> serial::Rx {
    let dp = pac::Peripheral::take().unwrap();
    let mut gpio = dp.GPIOF.split();
    let pf6 = gpio.pf6.into_alternate();
    let config = serial::config::default()
    .baudrate(9600)
    ;
    serial::Serial::rx(
    dp.UART7,
    pf6,
    config,
    clocks
    )
}

/// TODO
pub fn init_DMA1_Stream0(per: dma::traits::DMASet + dma::traits::PeriAddress, buf: embedded_dma::StaticWriteBuffer) -> dma::Transfer {
    let dp = pac::Peripherals::take().unwrap();
    let dma = dp.DMA1;
    let stream = dma::StreamsTuple::new(dma).0;
    let config = dma::config::DmaConfig::default()
    .priority(dma::config::Priority::Low)
    .memory_increment(true)
    ;
    dma::Transfer::init_memory_to_peripheral(
    stream,
    per,
    buf,
    None,
    config
    )
}

/// TODO
fn init_UART7_TX(cloks: rcc::Clocks) -> serial::Tx {
    let dp = pac::Peripheral::take().unwrap();
    let mut gpio = dp.GPIOF.split();
    let pf7 = gpio.pf7.into_alternate();
    let config = serial::config::default()
    .baudrate(9600)
    ;
    serial::Serial::tx(
    dp.UART7,
    pf7,
    config,
    clocks
    )
}

/// TODO
fn init_UART8_TX(cloks: rcc::Clocks) -> serial::Tx {
    let dp = pac::Peripheral::take().unwrap();
    let mut gpio = dp.GPIOE.split();
    let pe1 = gpio.pe1.into_alternate();
    let config = serial::config::default()
    .baudrate(1000000)
    ;
    serial::Serial::tx(
    dp.UART8,
    pe1,
    config,
    clocks
    )
}

/// TODO
pub fn init_DMA2_Stream6(per: dma::traits::DMASet + dma::traits::PeriAddress, buf: embedded_dma::StaticWriteBuffer) -> dma::Transfer {
    let dp = pac::Peripherals::take().unwrap();
    let dma = dp.DMA2;
    let stream = dma::StreamsTuple::new(dma).6;
    let config = dma::config::DmaConfig::default()
    .priority(dma::config::Priority::Low)
    .memory_increment(true)
    .fifo_enable(true)
    ;
    dma::Transfer::init_peripheral_to_memory(
    stream,
    per,
    buf,
    None,
    config
    )
}

/// TODO
pub fn init_DMA2_Stream1(per: dma::traits::DMASet + dma::traits::PeriAddress, buf: embedded_dma::StaticWriteBuffer) -> dma::Transfer {
    let dp = pac::Peripherals::take().unwrap();
    let dma = dp.DMA2;
    let stream = dma::StreamsTuple::new(dma).1;
    let config = dma::config::DmaConfig::default()
    .priority(dma::config::Priority::Low)
    .memory_increment(true)
    .peripheral_increment(true)
    .fifo_enable(true)
    ;
    dma::Transfer::init_memory_to_memory(
    stream,
    per,
    buf,
    None,
    config
    )
}

/// TODO
pub fn init_DMA2_Stream3(per: dma::traits::DMASet + dma::traits::PeriAddress, buf: embedded_dma::StaticWriteBuffer) -> dma::Transfer {
    let dp = pac::Peripherals::take().unwrap();
    let dma = dp.DMA2;
    let stream = dma::StreamsTuple::new(dma).3;
    let config = dma::config::DmaConfig::default()
    .priority(dma::config::Priority::Low)
    .memory_increment(true)
    .fifo_enable(true)
    ;
    dma::Transfer::init_memory_to_peripheral(
    stream,
    per,
    buf,
    None,
    config
    )
}