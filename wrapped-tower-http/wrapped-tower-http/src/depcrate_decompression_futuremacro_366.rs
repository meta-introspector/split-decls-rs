// Generated macro for macro_366 (macro)
macro_rules! Depcrate_decompression_futuremacro_366 {
() => {
// Module: crate::decompression::future
// Provides: {"macro_366"}
// Dependencies: {}
pin_project ! { # [doc = " Response future of [`Decompression`]."] # [doc = ""] # [doc = " [`Decompression`]: super::Decompression"] # [derive (Debug)] pub struct ResponseFuture < F > { # [pin] pub (crate) inner : F , pub (crate) accept : AcceptEncoding , } }
};
}
