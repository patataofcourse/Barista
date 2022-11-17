use std::env;

fn main() {
    cc::Build::new()
    .file("library/plgldr/src/plgldr.c")
    .include("library/plgldr/include")
    .include(env!("DEVKITPRO").to_owned() + "/libctru/include")
    .compile("plgldr")
}
