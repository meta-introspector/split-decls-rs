// Generated macro for is_nightly_channel (macro)
macro_rules! Depcrate_release_channelis_nightly_channel {
() => {
// Module: crate::release_channel
// Provides: {"is_nightly_channel"}
// Dependencies: {}
# [doc = " Checks if we're in a nightly build."] # [doc = ""] # [doc = " The environment variable `CFG_RELEASE_CHANNEL` is set during the rustc bootstrap"] # [doc = " to \"stable\", \"beta\", or \"nightly\" depending on what toolchain is being built."] # [doc = " If we are being built as part of the stable or beta toolchains, we want"] # [doc = " to disable unstable configuration options."] # [doc = ""] # [doc = " If we're being built by cargo (e.g., `cargo +nightly install rustfmt-nightly`),"] # [doc = " `CFG_RELEASE_CHANNEL` is not set. As we only support being built against the"] # [doc = " nightly compiler when installed from crates.io, default to nightly mode."] # [macro_export] macro_rules ! is_nightly_channel { () => { option_env ! ("CFG_RELEASE_CHANNEL") . map_or (true , | c | c == "nightly" || c == "dev") } ; }
};
}
