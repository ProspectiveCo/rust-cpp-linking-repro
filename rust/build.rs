pub fn main() {
    let output = cmake::Config::new("../cpp").build();

    println!("cargo:rustc-link-search=native={}", output.display());
    println!("cargo:rustc-link-lib=static=world");
}
