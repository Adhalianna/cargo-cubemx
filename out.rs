use stm32f4xx_hal::*;

/// TODO
fn init_USART2_RX(cloks: rcc::Clocks) -> serial::Rx {
    let dp = pac::Peripheral::take().unwrap();
    let mut gpio = dp.GPIOA.split();
    let pa3 = gpio.pa3.into_alternate();
    let config = serial::config::default()
    .baudrate(115200)
    ;
    serial::Serial::rx(
    dp.USART2,
    pa3,
    config,
    clocks
    )
}

/// TODO
pub fn init_DMA1_Stream5(per: dma::traits::DMASet + dma::traits::PeriAddress, buf: embedded_dma::StaticWriteBuffer) -> dma::Transfer {
    let dp = pac::Peripherals::take().unwrap();
    let dma = dp.DMA1;
    let stream = dma::StreamsTuple::new(dma).5;
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