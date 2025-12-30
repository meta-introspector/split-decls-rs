// Generated macro for macro_339 (macro)
macro_rules! Depcrate_decompression_bodymacro_339 {
() => {
// Module: crate::decompression::body
// Provides: {"macro_339"}
// Dependencies: {}
pin_project ! { # [doc = " Response body of [`RequestDecompression`] and [`Decompression`]."] # [doc = ""] # [doc = " [`RequestDecompression`]: super::RequestDecompression"] # [doc = " [`Decompression`]: super::Decompression"] pub struct DecompressionBody < B > where B : Body { # [pin] pub (crate) inner : BodyInner < B >, } }
};
}
