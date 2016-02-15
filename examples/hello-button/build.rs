extern crate wui_build;

fn main() {
    println!("cargo:rerun-if-changed=Cargo.toml");
    wui_build::guess().unwrap();
}
