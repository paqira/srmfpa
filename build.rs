fn main() {
    let mut builder = cc::Build::new();

    builder
        .file("src/c/common.c")
        .file("src/c/f32.c")
        .file("src/c/f64.c");

    #[cfg(feature = "f16-arithmetics")]
    builder.file("src/c/f16-a.c");
    #[cfg(feature = "f16-math")]
    builder.file("src/c/f16-m.c");

    #[cfg(feature = "f128-arithmetics")]
    builder.file("src/c/f128-a.c");
    #[cfg(feature = "f128-math")]
    builder.file("src/c/f128-m.c");

    builder.compile("c_impl");
}
