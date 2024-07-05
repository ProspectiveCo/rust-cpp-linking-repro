pub fn main() {
    // pyo3_build_config::add_extension_module_link_args();
    println!("cargo:rerun-if-changed=../cpp");

    let output = cmake::Config::new("../cpp").build();
    println!("cargo:rustc-link-search=all={}", output.display());
    println!("cargo:rustc-link-lib=static=world");

    cxx_build::bridge("src/bridge.rs")
        .include(output.join("include"))
        .flag_if_supported("-std=c++20")
        .flag("-fexceptions")
        .static_flag(true)
        .compile("worldbridge");

    println!("cargo:rustc-link-lib=static=worldbridge");
    println!("cargo:rerun-if-changed=src/bridge.rs");
}
