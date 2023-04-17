use std::process::Command;

fn main() {
    println!("cargo:rustc-link-search=native=library/plgldr/lib");
    println!("cargo:rustc-link-lib=static=plgldr");

    let output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output();
    let git_hash = match output {
        Ok(c) => String::from_utf8(c.stdout).unwrap_or(String::from("Invalid git output")),
        Err(_) => String::from("NOGIT"),
    };
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
}
