# BME280 bindings

This crate contains bindings to the [BME280 API by Bosch](https://github.com/boschsensortec/BME280_SensorAPI)

## How to use

- add library to your project
  ```
  myproj
    - Cargo.toml
    - Cargo.lock
    - src
        - main.rs
  bme280-bindings-rs    
  ```
- add dependency to the library with a local path

  ```toml
  # myproj/Cargo.toml
  [dependencies]
  ...
  bme280-bindings-rs = { path = "../bme280-bindings-rs" }
  ...  
  ```

- ensure you have the right C compiler for your target (ex. `arm-none-eabi-gcc` for Cortex M0 devices)
- set the required environment variables during compilation. this can be done in 2 ways:

  - by setting them in the cmd line

    ```bash
    $(myproj/) BME280_CFLAGS='' BME280_CC_INCLUDE='/usr/lib/gcc/arm-none-eabi/13.2.1/include' cargo build
    ```

  - by setting them in a `.cargo/config.toml` file

    ```toml
    # myproj/.cargo/config.toml
    [env]
    ...
    BME280_CFLAGS=''
    BME280_CC_INCLUDE='/usr/lib/gcc/arm-none-eabi/13.2.1/include'
    ...
    ```

