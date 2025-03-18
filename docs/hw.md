# Hardware

## Power budget

| Component | Current typ, mA | Current max, mA | Current stop mode, uA |
|:---:|:---:|:---:|:---:|
| MCU | - | 105 | 0.3 |
| LDO | - | - | 1 |
| memory chip | 0.01 | 25 | ~1 |
| audio amp | 4 | 4 + speaker current | 0 (disconnected) |

## Component selection

- `STM32L072CBT6` MCU
    - [Reference manual](stm32l0x2_reference_manual.pdf)
    - [Datasheet](stm32l072cb.pdf)
    - [JLCPCB part](https://jlcpcb.com/partdetail/STMicroelectronics-STM32L072CBT6/C465977)
    - has USB peripheral
- `BME280` pressure and temperature sensor 
    - [datasheet](./bst-bme280-ds002.pdf)
- small SMD speaker
    - [`NS4150B`](ULNS4150b_NSIWAY_0001.pdf)
- External flash to store audio: `W25Q128JVSIQ`
    - [Basic part in JLCPCB](https://jlcpcb.com/partdetail/WinbondElec-W25Q128JVSIQ/C97521)
    - SPI interface
    - 16 MiB ~= 3 min 16bit PCM audio@44.1 kHz
- flat li-po battery
    - [example battery](https://vikiwat.com/product/21044/akumulatorna-bateria-lp503040-3-7vdc-550mah-lipo.html?_gl=1*avbpzw*_up*MQ..*_gs*MQ..&gclid=Cj0KCQjwm7q-BhDRARIsACD6-fV4nz9wrJJZZHtHrbiDHF7CsGa15AtlHapZ-KCusHSsi9U_Mh-u0QsaAskiEALw_wcB) - 550 mAh
        Standard charge is 0.2C, so ~110 mA.
    - [TP4054](https://jlcpcb.com/partdetail/Goodwork-TP4054/C21713960) charge IC - [datasheet](tp4054.pdf)
        `R_prog` = 1000/0.11 ~= 9k, 10k in schematic
    - [LDO](https://jlcpcb.com/partdetail/TorexSemicon-XC6206P332MRG/C5446) - [datasheet](./ldo_datasheet.pdf)
