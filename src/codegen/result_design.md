# Remarks

Since HAL crates vary between each other greatly, the compiler will be first implemented for f4 family.
# HAL imports

```rs
//use <familyname>xx_hal::*;
use stm32f4xx_hal::*;
```

# RCC configuration

## Sample 1

```
RCC.SAIAFreq_Value=96000000
RCC.SAIBFreq_Value=96000000
RCC.HSE_VALUE=8000000
RCC.AHBFreq_Value=180000000
RCC.SYSCLKFreq_VALUE=180000000
```

```rs
pub fn configure_clocks( /*...*/ ) -> rcc::Clocks {
  let dp = pac::Peripherals::take().unwrap();
  let mut rcc = dp.RCC.constrain();
  //rcc.cfgr.<clock name>_clk(<freq>);
  rcc.cfgr.saia_clk(96000000); //RCC.SAIAFreq_Value=96000000
  rcc.cfgr.saib_clk(96000000); //RCC.SAIBFreq_Value=96000000
  rcc.cfgr.use_hse(8000000); //RCC.HSE_VALUE=8000000
  rcc.cfgr.hclk(180000000); //RCC.AHBFreq_Value=180000000
  rcc.cfgr.sysclk(180000000); //RCC.SYSCLKFreq_VALUE=180000000
  rcc.cfgr.freeze()
}
```

# USART and DMA configuration

## Sample 1
```
Dma.USART2_RX.0.Direction=DMA_PERIPH_TO_MEMORY
Dma.USART2_RX.0.FIFOMode=DMA_FIFOMODE_DISABLE
Dma.USART2_RX.0.Instance=DMA1_Stream5
Dma.USART2_RX.0.MemDataAlignment=DMA_MDATAALIGN_BYTE
Dma.USART2_RX.0.MemInc=DMA_MINC_ENABLE
Dma.USART2_RX.0.Mode=DMA_NORMAL
Dma.USART2_RX.0.PeriphDataAlignment=DMA_PDATAALIGN_BYTE
Dma.USART2_RX.0.PeriphInc=DMA_PINC_DISABLE
Dma.USART2_RX.0.Priority=DMA_PRIORITY_LOW
Dma.USART2_RX.0.RequestParameters=Instance,Direction,PeriphInc,MemInc,PeriphDataAlignment,MemDataAlignment,Mode,Priority,FIFOMode
USART2.BaudRate=115200
USART2.IPParameters=VirtualMode,BaudRate
USART2.VirtualMode=VM_ASYNC
PA3.GPIOParameters=GPIO_Label
PA3.GPIO_Label=PA3_USART2_RX_M_TEL1
PA3.Mode=Asynchronous
PA3.Signal=USART2_RX
```

```rs
//pub fn configure_<serial name>( clocks: Clocks) -> serial::<mode from pin>
pub fn configure_usart2(clocks: Clocks) -> serial::Rx {
  let dp = pac::Peripherals::take().unwrap();
  let mut gpioa = dp.GPIOA.split();
  let pa3 = gpioa.pa3.into_alternate();

  let config = serial::config::default().baudrate(115200); //USART2.BaudRate=115200
  config.dma = serial::config::DmaConfig::Rx; //PA3.Signal=USART2_RX

  serial::Serial::rx(
    dp.USART2,
    pa3,
    config,
    clocks
  )
}
```

```rs
pub fn configure_channel_0(per: DMASet + PeriAddress, buf: embedded_dma::StaticWriteBuffer) -> dma::Transfer {
  let dp = pac::Peripherals::take().unwrap();
  let dma = dp.DMA1; //Dma.USART2_RX.0.Instance=DMA1_Stream5
  let stream = dma::StreamsTuple::new(dma).5; //Dma.USART2_RX.0.Instance=DMA1_Stream5
  let config = dma::config::DmaConfig::default()
                  .priority(dma::config::Priority::Low) //Dma.USART2_RX.0.Priority=DMA_PRIORITY_LOW
                  .memory_increment(true) //Dma.USART2_RX.0.MemInc=DMA_MINC_ENABLE
                  .peripheral_increment(false) //Dma.USART2_RX.0.PeriphInc=DMA_PINC_DISABLE
                  .fifo_enable(false) //Dma.USART2_RX.0.FIFOMode=DMA_FIFOMODE_DISABLE

  dma::Transfer::init_peripheral_to_memory( //Dma.USART2_RX.0.Direction=DMA_PERIPH_TO_MEMORY
      stream,
      per,
      buf,
      None,
      config
    )
}
```