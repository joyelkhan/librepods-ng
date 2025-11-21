fn main() {
    println!("cargo:rerun-if-changed=crates/core/src");
    println!("cargo:rerun-if-changed=crates/ffi/src");
}
