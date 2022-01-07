extern crate rustc_version;
use rustc_version::{version_meta, Channel};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let is_nightly = version_meta().expect("nightly check failed").channel == Channel::Nightly;
    let asm_enabled = cfg!(feature = "asm");

    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_features = std::env::var("CARGO_CFG_TARGET_FEATURE").unwrap_or("".into());
    let target_features = target_features.split(",").collect::<Vec<_>>();
    let bmi2_enabled = target_features.contains(&"bmi2");
    let adx_enabled = target_features.contains(&"adx");

    let is_x86_64 = target_arch == "x86_64";
    let should_use_asm = asm_enabled && bmi2_enabled && adx_enabled && is_x86_64 && is_nightly;
    if should_use_asm {
        println!("cargo:rustc-cfg=use_asm");
    }
}
