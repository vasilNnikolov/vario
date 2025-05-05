/// cleans artifacts from any previous builds of the c library, compiles it, and generates rust bindings to it with bindgen
fn compile_link_c_lib() {
    use std::path::PathBuf;
    let bindings_path = PathBuf::from("./src/bindings.rs");
    let _ = std::fs::remove_file(&bindings_path);

    let libdir_path = PathBuf::from("bosch-api").canonicalize().unwrap();

    cc::Build::new()
        .no_default_flags(true)
        .cargo_debug(true)
        .file(libdir_path.join("BME280_SensorAPI/bme280.c"))
        .include(libdir_path.join("BME280_SensorAPI"))
        .compiler("arm-none-eabi-gcc")
        .archiver("arm-none-eabi-ar")
        .compile("bme280");

    let libdir_path = PathBuf::from("bosch-api").canonicalize().unwrap();

    // TODO see which env variables are set in the bme280 headers and expose them as features or sth
    // generate the rust bindings for the c library
    let headers_path = libdir_path.join("wrapper.h");
    let bindings = bindgen::Builder::default()
        .use_core()
        .fit_macro_constants(true)
        .clang_macro_fallback()
        .derive_default(true)
        .clang_arg("-I/usr/lib/gcc/arm-none-eabi/13.2.1/include") // TODO set include some other way
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
