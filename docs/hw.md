# Hardware

## Power budget

| Component | Current typ, mA | Current max, mA |
|:---:|:---:|:---:|
| MCU | - | 105 | 
| memory chip | 0.01 | 25 |
| audio amp | 4 | 4 + speaker current |

## Component selection

- `STM32L072CBT6` MCU
    - [Reference manual](stm32l0x2_reference_manual.pdf)
    - [Datasheet](stm32l072cb.pdf)
    - [JLCPCB part](https://jlcpcb.com/partdetail/STMicroelectronics-STM32L072CBT6/C465977)
    - has USB peripheral

- `BME280` pressure and temperature sensor
- small SMD speaker
    - [`NS4150B`](ULNS4150b_NSIWAY_0001.pdf)
- External flash to store audio: `W25Q128JVSIQ`
    - [Basic part in JLCPCB](https://jlcpcb.com/partdetail/WinbondElec-W25Q128JVSIQ/C97521)
    - SPI interface
    - 16 MiB ~= 3 min 16bit PCM audio@44.1 kHz
- coin battery
    - `LIR2032` - 3.6V, 45 mAh, rechargeable
    - [TP4054](https://jlcpcb.com/partdetail/Goodwork-TP4054/C21713960) charge IC - [datasheet](tp4054.pdf)
    - [LDO](https://jlcpcb.com/partdetail/TorexSemicon-XC6206P332MRG/C5446) - [datasheet](./ldo_datasheet.pdf)
