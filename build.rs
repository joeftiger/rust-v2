fn main() {
    #[cfg(not(any(feature = "spectral", feature = "srgb", feature = "xyz")))]
    compile_error!("One feature must be enabled: 'spectral', 'srgb' or 'xyz'");
    #[cfg(any(
        all(feature = "spectral", any(feature = "srgb", feature = "xyz")),
        all(feature = "srgb", any(feature = "spectral", feature = "xyz")),
        all(feature = "xyz", any(feature = "spectral", feature = "srgb")),
    ))]
    compile_error!("Only one feature must be enabled: 'spectral', 'srgb' or 'xyz'");
}
