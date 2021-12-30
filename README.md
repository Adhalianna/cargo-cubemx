# cargo-cubemx

The software is currently developed as a part of Formal Languages and Compilers
project. Because of the assignment constraints (primarily time constraints), neither performance nor full
functionality coverage are targeted at the moment.

## FLC project goals

* parse the .ioc file
* compile to Rust using the following types of configuration:
  * DMA related
  * Serial ports related (USART)
  * MCU family
* Support the following targets:
  * STM32F4 family and its HAL crates