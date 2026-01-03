fn main() {
    // Compile Slint UI into Rust bindings at build time.
    slint_build::compile("ui/app.slint").unwrap();
}
