extern crate cc;

fn main() {
    let src = std::env::current_dir().unwrap();
    let flips = src.join("flips");

    println!("cargo:include={}", flips.display());

    println!("cargo:rustc-link-lib=ips");
    cc::Build::new()
        .cpp(true)
        .include("flips")
        .warnings(true)
        .file("flips/libips.cpp")
        .compile("ips");

    println!("cargo:rustc-link-lib=ups");
    cc::Build::new()
        .include("flips")
        .warnings(false)
        .file("flips/libups.cpp")
        .compile("libups.a");

    println!("cargo:rustc-link-lib=bps");
    cc::Build::new()
        .include("flips")
        .warnings(false)
        .file("flips/crc32.cpp")
        .file("flips/libbps.cpp")
        .file("flips/libbps-suf.cpp")
        .compile("libbps.a");
}
