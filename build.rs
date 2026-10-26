use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_config = manifest_dir.join("../../../wkg-config.toml");

    println!("cargo:rerun-if-changed=wit/world.wit");
    println!("cargo:rerun-if-changed=wkg.toml");
    println!("cargo:rerun-if-changed=wkg.lock");
    println!("cargo:rerun-if-changed={}", workspace_config.display());
    println!("cargo:rerun-if-env-changed=WKG_CONFIG_FILE");

    let config_file = std::env::var_os("WKG_CONFIG_FILE")
        .map(PathBuf::from)
        .unwrap_or(workspace_config);

    let status = std::process::Command::new("wkg")
        .args(["wit", "fetch"])
        .env("WKG_CONFIG_FILE", config_file)
        .status()
        .expect("Failed to run `wkg wit fetch`. `wkg` is likely not installed.");

    if !status.success() {
        println!("cargo:warning=`wkg wit fetch` failed. Using existing WIT deps if available.");
    }
}
