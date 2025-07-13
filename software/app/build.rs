//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the
//! new memory settings.
//!
//! The build script also sets the linker flags to tell it which link script to use.

use chrono::{self, Datelike, Timelike};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

/// sets constants equal to the compile time
fn set_compile_time() {
    let now = chrono::Local::now();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("compiled_time.rs");

    std::fs::write(
        &dest_path,
        format!(
            "mod build_time {{
pub const YEAR: u8= {};
pub const MONTH: u8 = {};
pub const DAY: u8 = {};
pub const WEEKDAY: u32 = {};
pub const HOUR: u8 = {};
pub const MINUTE: u8 = {};
pub const SECOND: u8 = {};
}}",
            now.year() % 100,
            now.month(),
            now.day(),
            now.weekday().number_from_monday(),
            now.hour(),
            now.minute(),
            now.second()
        ),
    )
    .unwrap();
}

fn main() {
    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // println!("cargo:rerun-if-changed=memory.x");

    // Specify linker arguments.

    // `--nmagic` is required if memory section addresses are not aligned to 0x10000,
    // for example the FLASH and RAM sections in your `memory.x`.
    // See https://github.com/rust-embedded/cortex-m-quickstart/pull/95
    println!("cargo:rustc-link-arg=--nmagic");

    // Set the linker script to the one provided by cortex-m-rt.
    println!("cargo:rustc-link-arg=-Tlink.x");
    println!("cargo:rustc-link-arg=-Tdefmt.x"); // necessary for the defmt crate

    set_compile_time();
}
