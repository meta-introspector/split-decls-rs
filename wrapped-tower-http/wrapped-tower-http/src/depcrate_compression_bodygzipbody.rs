// Generated macro for GzipBody (type)
macro_rules! Depcrate_compression_bodyGzipBody {
() => {
// Module: crate::compression::body
// Provides: {"GzipBody"}
// Dependencies: {}
# [cfg (feature = "compression-gzip")] type GzipBody < B > = WrapBody < GzipEncoder < B > > ;
};
}
