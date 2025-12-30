// Generated macro for RequestDecompression (struct)
macro_rules! Depcrate_decompression_request_serviceRequestDecompression {
() => {
// Module: crate::decompression::request::service
// Provides: {"RequestDecompression"}
// Dependencies: {}
# [doc = " Decompresses request bodies and calls its underlying service."] # [doc = ""] # [doc = " Transparently decompresses request bodies based on the `Content-Encoding` header."] # [doc = " When the encoding in the `Content-Encoding` header is not accepted an `Unsupported Media Type`"] # [doc = " status code will be returned with the accepted encodings in the `Accept-Encoding` header."] # [doc = ""] # [doc = " Enabling pass-through of unaccepted encodings will not return an `Unsupported Media Type` but"] # [doc = " will call the underlying service with the unmodified request if the encoding is not supported."] # [doc = " This is disabled by default."] # [doc = ""] # [doc = " See the [module docs](crate::decompression) for more details."] # [derive (Debug , Clone)] pub struct RequestDecompression < S > { pub (super) inner : S , pub (super) accept : AcceptEncoding , pub (super) pass_through_unaccepted : bool , }
};
}
