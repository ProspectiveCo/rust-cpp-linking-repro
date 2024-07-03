pub fn main() {
    println!("cargo:rerun-if-changed=../cpp");

    let output = cmake::Config::new("../cpp").build();
    println!("cargo:rustc-link-search=native={}", output.display());
    println!("cargo:rustc-link-lib=static=world");

    cxx_build::bridge("src/bridge.rs")
        .include(output.join("include"))
        .flag_if_supported("-std=c++20")
        .flag("-fexceptions")
        .compile("worldbridge");
    
    println!("cargo:rerun-if-changed=src/bridge.rs");
}
