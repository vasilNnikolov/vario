fn compile_link_c_lib() {
    use std::path::PathBuf;
    let bindings_path = PathBuf::from("./src/bindings.rs");
    // let _ = std::fs::remove_file(&bindings_path);

    let libdir_path = PathBuf::from("bosch-api").canonicalize().unwrap();

    let compiler = std::env::var("BME280_CC")
        .expect("The BME280_CC environment variable must be set to compile the BME280 drivers.\nExample: BME280_CC=arm-none-eabi-gcc");
    let archiver = std::env::var("BME280_AR")
        .expect("The BME280_AR environment variable must be set to archive the BME280 drivers into a static library.\nExample: BME280_AR=arm-none-eabi-ar");

    cc::Build::new()
        .no_default_flags(true)
        .try_flags_from_environment("BME280_CFLAGS").expect("The env variable BME280_CFLAGS must be set to any necessary compiler flags needed to compile the crate")
        .cargo_debug(true)
        .file(libdir_path.join("BME280_SensorAPI/bme280.c"))
        .include(libdir_path.join("BME280_SensorAPI"))
        .compiler(compiler)
        .archiver(archiver)
        .compile("bme280");

    let cc_include_path = std::env::var("BME280_CC_INCLUDE").expect(
        "The BME280_CC_INCLUDE env variable must be set to the path to stdint.h and stddef.h\nExample: BME280_CC_INCLUDE='/usr/lib/gcc/arm-none-eabi/13.2.1/include'",
    );

    let libdir_path = PathBuf::from("bosch-api").canonicalize().unwrap();

    // TODO see which env variables are set in the bme280 headers and expose them as features or sth
    // generate the rust bindings for the c library
    let headers_path = libdir_path.join("wrapper.h");
    let bindings = bindgen::Builder::default()
        .use_core()
        .fit_macro_constants(true)
        .clang_macro_fallback()
        .derive_default(true)
        .clang_arg(format!("-I{}", cc_include_path)) // TODO set include some other way
        .header(headers_path.to_str().unwrap())
        .raw_line("#![allow(non_snake_case)]") // add these to avoid rust warnings
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_upper_case_globals)]")
        .generate_comments(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(bindings_path)
        .expect("Couldn't write bindings!");
}

pub fn main() {
    compile_link_c_lib();
}
