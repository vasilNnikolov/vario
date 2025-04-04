# Hardware

## Power budget

| Component | Current typ, mA | Current max, mA | Current stop mode, uA |
|:---:|:---:|:---:|:---:|
| MCU | - | 105 | 0.3 |
| LDO | - | - | 1 |
| memory chip | 0.01 | 25 | ~1 |
| audio amp | 4 | 4 + speaker current | 0 (disconnected) |

## Component selection

### `STM32L072CBT6` MCU

Low power MCU with USB peripheral

- [Reference manual](./stm32l0x2_reference_manual.pdf)
- [Datasheet](./stm32l072cb.pdf)
- [HW design guide](./an4467-getting-started-with-stm32l0xx-hardware-development-stmicroelectronics.pdf)
- [JLCPCB part](https://jlcpcb.com/partdetail/STMicroelectronics-STM32L072CBT6/C465977)


### Oscillators

[design guide](./an2867-guidelines-for-oscillator-design-on-stm8afals-and-stm32-mcusmpus-stmicroelectronics.pdf)

#### LSE 

[LSE](https://www.lcsc.com/product-detail/Crystals_Seiko-Epson-X1A0000610006_C99009.html)

According to design guide section 3.4, `g_m_crit(max) = 1.48 uA/V`. With HIGH drive strength of the oscillator, the gain margin is ~9

#### HSE

[HSE](https://www.lcsc.com/product-detail/Crystals_YXC-Crystal-Oscillators-X322516MLB4SI_C13738.html)

`g_m_crit(max) = 0.46 mA/V`. Gain margin ~= 16

### `BME280` pressure sensor

- [datasheet](./bst-bme280-ds002.pdf)
- [C driver](https://github.com/boschsensortec/BME280_SensorAPI)
- [rust driver](https://docs.rs/bme280/latest/bme280/)

## Other components

- small SMD speaker
    - [`NS4150B`](./ULNS4150b_NSIWAY_0001.pdf)
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

