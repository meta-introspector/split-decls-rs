// Generated macro for macro_189 (macro)
macro_rules! Depcrate_compression_bodymacro_189 {
() => {
// Module: crate::compression::body
// Provides: {"macro_189"}
// Dependencies: {}
pin_project ! { # [doc = " Response body of [`Compression`]."] # [doc = ""] # [doc = " [`Compression`]: super::Compression"] pub struct CompressionBody < B > where B : Body , { # [pin] pub (crate) inner : BodyInner < B >, } }
};
}
