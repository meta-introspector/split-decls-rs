// Generated macro for macro_196 (macro)
macro_rules! Depcrate_compression_bodymacro_196 {
() => {
// Module: crate::compression::body
// Provides: {"macro_196"}
// Dependencies: {}
pin_project_cfg ! { # [project = BodyInnerProj] pub (crate) enum BodyInner < B > where B : Body , { # [cfg (feature = "compression-gzip")] Gzip { # [pin] inner : GzipBody < B >, } , # [cfg (feature = "compression-deflate")] Deflate { # [pin] inner : DeflateBody < B >, } , # [cfg (feature = "compression-br")] Brotli { # [pin] inner : BrotliBody < B >, } , # [cfg (feature = "compression-zstd")] Zstd { # [pin] inner : ZstdBody < B >, } , Identity { # [pin] inner : B , } , } }
};
}
