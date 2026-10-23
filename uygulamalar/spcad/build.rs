//! Windows only: embed the app icon and version info into `spcad.exe`. Elsewhere a no-op.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../varliklar/uygulama-simgesi/spcad.ico");
    println!("cargo:rerun-if-env-changed=SPCAD_REQUIRE_WINRES");
    println!("cargo:rerun-if-env-changed=SPCAD_BUILD_SHA");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    if std::path::Path::new("../../varliklar/uygulama-simgesi/spcad.ico").exists() {
        res.set_icon("../../varliklar/uygulama-simgesi/spcad.ico");
    }
    res.set("ProductName", "SP-GROUP CAD")
        .set("FileDescription", "SP-GROUP CAD computer-aided design")
        .set("CompanyName", "SP-GROUP")
        .set("LegalCopyright", "Copyright (c) 2026 ArtCraft Team and the CADCraft contributors. Modified by SP-GROUP. MIT OR Apache-2.0.")
        .set("OriginalFilename", "spcad.exe")
        .set("InternalName", "spcad");
    if let Err(e) = res.compile() {
        if std::env::var_os("SPCAD_REQUIRE_WINRES").is_some() {
            eprintln!("embedding Windows resources failed: {e}");
            std::process::exit(1);
        }
        println!("cargo:warning=spcad.exe built without icon/version resources: {e}");
    }
}
