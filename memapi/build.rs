use cc;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rerun-if-changed=src/filpreload.c");
    println!("cargo:rustc-link-search=native={}", out_dir);
    cc::Build::new()
        .file("src/filpreload.c")
        .warnings_into_errors(true)
        .flag("-std=c11")
        .flag("-Wall")
        .flag("-Werror=format-security")
        .flag("-Werror=implicit-function-declaration")
        .static_flag(true)
        .shared_flag(false)
        .flag("-fvisibility=hidden")
        .cargo_metadata(false)
        .compile("filpreload");
}
