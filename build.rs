fn main() {
    // Re-run if build script changes
    println!("cargo:rerun-if-changed=build.rs");
}
