use cc;

fn main() {
    cc::Build::new()
        .file("src/filpreload.c")
        .warnings_into_errors(true)
        .flag("-std=c11")
        .flag("-Wall")
        .flag("-Werror=format-security")
        .flag("-Werror=implicit-function-declaration")
        .compile("filpreload");
    println!("cargo:rerun-if-changed=src/filpreload.c");
}
