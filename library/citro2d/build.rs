

use std::env;

fn main() {
    let dkp_path = env::var("DEVKITPRO").unwrap();
    let debug_symbols = env::var("DEBUG").unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=DEVKITPRO");
    println!("cargo:rustc-link-search=native={}/libctru/lib", dkp_path);
    println!(
        "cargo:rustc-link-lib=static={}",
        match debug_symbols.as_str() {
            // Based on valid values described in
            // https://doc.rust-lang.org/cargo/reference/profiles.html#debug
            "0" | "false" => "citro2d",
            _ => "citro2dd",
        }
    );
    println!(
        "cargo:rustc-link-lib=static={}",
        match debug_symbols.as_str() {
            // Based on valid values described in
            // https://doc.rust-lang.org/cargo/reference/profiles.html#debug
            "0" | "false" => "citro3d",
            _ => "citro3dd",
        }
    );
}