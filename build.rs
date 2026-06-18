fn main() {
    println!("cargo:rerun-if-changed=wit/world.wit");
    println!("cargo:rerun-if-changed=wkg.toml");

    let status = std::process::Command::new("wkg")
        .args(["wit", "fetch"])
        .status()
        .expect("failed to run `wkg wit fetch` — is wkg installed?");

    if !status.success() {
        println!("cargo:warning=wkg wit fetch failed — using existing WIT deps if available");
    }
}
