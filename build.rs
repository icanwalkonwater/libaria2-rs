fn main() {
    // Generate CXX bridge
    cxx_build::bridge("src/lib.rs")
        .file("src/aria2_bridge.cpp")
        .flag_if_supported("-std=c++14")
        .compile("aria2_bridge");

    // Link with aria2
    println!("cargo:rustc-link-lib=aria2");

    // Regen bridge if these files changes
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/aria2_bridge.cpp");
    println!("cargo:rerun-if-changed=include/aria2_bridge.hpp");
}
