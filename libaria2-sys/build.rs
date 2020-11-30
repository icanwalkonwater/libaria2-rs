fn main() {
    // Link with aria2
    pkg_config::Config::new()
        .atleast_version("1.35.0")
        .probe("libaria2")
        .expect("Dependency not satisfied: aria2 can't be found !");
    println!("cargo:rustc-link-lib=aria2");

    // Generate CXX bridge
    let mut bridge = cxx_build::bridge("src/lib.rs");

    bridge
        .file("src/aria2_bridge.cpp")
        .flag_if_supported("-std=c++14");

    if cfg!(debug_assertions) {
        bridge.flag_if_supported("-O3");
    }

    bridge.compile("aria2_bridge");

    // Regen bridge if these files changes
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/aria2_bridge.cpp");
    println!("cargo:rerun-if-changed=include/aria2_bridge.hpp");
    println!("cargo:rerun-if-changed=include/DownloadHandleWrapper.hpp");
}
