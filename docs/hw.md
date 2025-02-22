## Hardware

- `STM32L071CBT6` MCU
    - [Reference manual](/stm32l0x1reference_manual.pdf)
    - [Datasheet](/stm32l071v8.pdf)

- `BME280` pressure and temperature sensor
- small SMD speaker
- External flash to store audio: `W25Q128JVSIQ`
    - [Basic part in JLCPCB](https://jlcpcb.com/partdetail/WinbondElec-W25Q128JVSIQ/C97521)
    - SPI interface
    - 16 MiB ~= 3 min 16bit PCM audio@44.1 kHz
- coin battery
    - `LIR2032` - 3.6V, 45 mAh, rechargeable
