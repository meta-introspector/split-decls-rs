// Generated macro for RequestDecompressionLayer (struct)
macro_rules! Depcrate_decompression_request_layerRequestDecompressionLayer {
() => {
// Module: crate::decompression::request::layer
// Provides: {"RequestDecompressionLayer"}
// Dependencies: {}
# [doc = " Decompresses request bodies and calls its underlying service."] # [doc = ""] # [doc = " Transparently decompresses request bodies based on the `Content-Encoding` header."] # [doc = " When the encoding in the `Content-Encoding` header is not accepted an `Unsupported Media Type`"] # [doc = " status code will be returned with the accepted encodings in the `Accept-Encoding` header."] # [doc = ""] # [doc = " Enabling pass-through of unaccepted encodings will not return an `Unsupported Media Type`. But"] # [doc = " will call the underlying service with the unmodified request if the encoding is not supported."] # [doc = " This is disabled by default."] # [doc = ""] # [doc = " See the [module docs](crate::decompression) for more details."] # [derive (Debug , Default , Clone)] pub struct RequestDecompressionLayer { accept : AcceptEncoding , pass_through_unaccepted : bool , }
};
}
