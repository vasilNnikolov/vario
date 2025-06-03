# Vario main application

**NOTE** this crate is based on [cortex-m quickstart](https://github.com/rust-embedded/cortex-m-quickstart/tree/master) and the [embedded rust book](https://docs.rust-embedded.org/book/intro/index.html)

## Flashing instructions

The flashing has been tested with an [ST-Link V2](https://www.st.com/en/development-tools/st-link-v2.html). To program and debug the board (R1.0), connect

- board VCC to ST-Link VAPP (pin 1)
- board SWDIO to ST-Link SW IO (pin 7)
- board SWCLK to ST-Link SW CLK (pin 9)
- board NRST to ST-Link NRST (pin 15)
- board GND to ST-Link GND (pin 20)

For reference, see the [ST-Link user manual](./docs/um1075-stlinkv2-incircuit-debuggerprogrammer-for-stm8-and-stm32-stmicroelectronics.pdf)

**NOTE:** with the configuration above, the ST-Link does *NOT* power the board. To connect successfully, you must either:

- power the MCU externally (with USB or battery)
- connect the board VCC to ST-Link VDD (pin 19)

