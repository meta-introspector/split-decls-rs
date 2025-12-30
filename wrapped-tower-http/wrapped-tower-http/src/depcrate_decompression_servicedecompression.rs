// Generated macro for Decompression (struct)
macro_rules! Depcrate_decompression_serviceDecompression {
() => {
// Module: crate::decompression::service
// Provides: {"Decompression"}
// Dependencies: {}
# [doc = " Decompresses response bodies of the underlying service."] # [doc = ""] # [doc = " This adds the `Accept-Encoding` header to requests and transparently decompresses response"] # [doc = " bodies based on the `Content-Encoding` header."] # [doc = ""] # [doc = " See the [module docs](crate::decompression) for more details."] # [derive (Debug , Clone)] pub struct Decompression < S > { pub (crate) inner : S , pub (crate) accept : AcceptEncoding , }
};
}
