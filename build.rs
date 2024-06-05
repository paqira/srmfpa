fn main() {
    let mut builder = cc::Build::new();

    builder
        .std("c11") // require c11 as default
        .file("src/c/common.c")
        .file("src/c/f32.c")
        .file("src/c/f64.c");

    #[cfg(feature = "f16")]
    builder.file("src/c/f16.c");

    #[cfg(feature = "f128")]
    builder.file("src/c/f128.c");

    builder.compile("c_impl");
}
