// Generated macro for macro_351 (macro)
macro_rules! Depcrate_decompression_bodymacro_351 {
() => {
// Module: crate::decompression::body
// Provides: {"macro_351"}
// Dependencies: {}
pin_project ! { # [project = BodyInnerProj] pub (crate) enum BodyInner < B > where B : Body , { Gzip { # [pin] inner : GzipBody < B >, } , Deflate { # [pin] inner : DeflateBody < B >, } , Brotli { # [pin] inner : BrotliBody < B >, } , Zstd { # [pin] inner : ZstdBody < B >, } , Identity { # [pin] inner : B , } , } }
};
}
