// Generated macro for Compression (struct)
macro_rules! Depcrate_compression_serviceCompression {
() => {
// Module: crate::compression::service
// Provides: {"Compression"}
// Dependencies: {}
# [doc = " Compress response bodies of the underlying service."] # [doc = ""] # [doc = " This uses the `Accept-Encoding` header to pick an appropriate encoding and adds the"] # [doc = " `Content-Encoding` header to responses."] # [doc = ""] # [doc = " See the [module docs](crate::compression) for more details."] # [derive (Clone , Copy)] pub struct Compression < S , P = DefaultPredicate > { pub (crate) inner : S , pub (crate) accept : AcceptEncoding , pub (crate) predicate : P , pub (crate) quality : CompressionLevel , }
};
}
