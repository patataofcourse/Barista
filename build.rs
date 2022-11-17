use std::env;

fn main() {
    println!("cargo:rustc-link-search=native=library/plgldr/lib");
    println!("cargo:rustc-link-lib=static=plgldr")
}
