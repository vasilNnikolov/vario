# STM32-based vario

## Hardware 

- `STM32L071CBT6` MCU
  - [Reference manual](https://www.google.com/url?sa=t&source=web&rct=j&opi=89978449&url=https://www.st.com/resource/en/reference_manual/rm0377-ultralowpower-stm32l0x1-advanced-armbased-32bit-mcus-stmicroelectronics.pdf&ved=2ahUKEwi2oNa_5M2LAxX2RfEDHUMhICYQFnoECBYQAQ&usg=AOvVaw29XzblTsaLMdrlTpNE_gpY)
  - [Datasheet](https://www.lcsc.com/datasheet/lcsc_datasheet_1809301218_STMicroelectronics-STM32L071CBT6_C79758.pdf)
- `BME280` pressure&temperature sensor
- small SMD speaker
- coin battery

## Software

- project structure from the [embedded rust book](https://docs.rust-embedded.org/book/intro/index.html)
- PAC/HAL from [this repo](https://github.com/stm32-rs/stm32-rs)
