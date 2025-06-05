use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=./build.rs");
    println!("cargo:rerun-if-changed=./STM32CubeL0");

    let bindings = bindgen::Builder::default()
        .use_core()
        .clang_macro_fallback()
        .derive_default(true)
        .clang_arg("-I./STM32CubeL0/Drivers/CMSIS/Include")
        .header("./STM32CubeL0/Drivers/CMSIS/Device/ST/STM32L0xx/Include/stm32l072xx.h")
        // .raw_line("#![allow(non_snake_case)]") // add these to avoid rust warnings
        // .raw_line("#![allow(non_camel_case_types)]")
        // .raw_line("#![allow(non_upper_case_globals)]")
        // .raw_line("#![allow(unsafe_op_in_unsafe_fn)]")
        .generate_comments(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    bindings.write_to_file(out.join("c_bindings.rs")).unwrap();
}
