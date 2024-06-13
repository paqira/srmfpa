fn main() {
    let mut builder = cc::Build::new();

    builder
        .file("src/c/rounding_mode.c")
        .file("src/c/f32.c")
        .file("src/c/f64.c");

    #[cfg(feature = "f16")]
    builder.file("src/c/f16.c");

    #[cfg(feature = "f128")]
    builder.file("src/c/f128.c");

    builder.compile("c_impl");

    println!("build-command:{:?}", builder.get_compiler().to_command());
}
