extern crate cc;

fn main() {
    // get the current and output directories
    let cwd = std::env::current_dir().unwrap();
    let out = std::env::var("OUT_DIR").unwrap();
    let ref src = cwd.join("src");
    let ref flips = cwd.join("flips");
    let ref patched = std::path::PathBuf::from(out).join("flips");

    // copy C++ sources refering to `crc32.h` locally to a different folder
    // to force them to use the one we defined in `src`.
    std::fs::create_dir_all(patched).ok();
    for name in &["libups.cpp", "libbps.cpp", "libbps-suf.cpp"] {
        std::fs::copy(flips.join(name), patched.join(name)).unwrap();
    }

    // build `lipips`
    println!("cargo:rustc-link-lib=ips");
    cc::Build::new()
        .cpp(true)
        .include(flips)
        .warnings(true)
        .file(flips.join("libips.cpp"))
        .compile("ips");

    // build `lipups`
    println!("cargo:rustc-link-lib=ups");
    cc::Build::new()
        .cpp(true)
        .include(src)
        .include(flips)
        .warnings(false)
        .file(patched.join("libups.cpp"))
        .compile("libups.a");

    // build `lipbps`
    println!("cargo:rustc-link-lib=bps");
    cc::Build::new()
        .cpp(true)
        .include(src)
        .include(flips)
        .warnings(false)
        .file(flips.join("divsufsort.c"))
        .file(patched.join("libbps.cpp"))
        .file(patched.join("libbps-suf.cpp"))
        .compile("libbps.a");
}
