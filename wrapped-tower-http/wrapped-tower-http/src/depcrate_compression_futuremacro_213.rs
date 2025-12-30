// Generated macro for macro_213 (macro)
macro_rules! Depcrate_compression_futuremacro_213 {
() => {
// Module: crate::compression::future
// Provides: {"macro_213"}
// Dependencies: {}
pin_project ! { # [doc = " Response future of [`Compression`]."] # [doc = ""] # [doc = " [`Compression`]: super::Compression"] # [derive (Debug)] pub struct ResponseFuture < F , P > { # [pin] pub (crate) inner : F , pub (crate) encoding : Encoding , pub (crate) predicate : P , pub (crate) quality : CompressionLevel , } }
};
}
