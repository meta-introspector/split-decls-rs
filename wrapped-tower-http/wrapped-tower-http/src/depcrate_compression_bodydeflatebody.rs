// Generated macro for DeflateBody (type)
macro_rules! Depcrate_compression_bodyDeflateBody {
() => {
// Module: crate::compression::body
// Provides: {"DeflateBody"}
// Dependencies: {}
# [cfg (feature = "compression-deflate")] type DeflateBody < B > = WrapBody < ZlibEncoder < B > > ;
};
}
