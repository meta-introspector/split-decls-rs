// Generated macro for impl_169 (impl)
macro_rules! Depcrate_compression_predicateimpl_169 {
() => {
// Module: crate::compression::predicate
// Provides: {"impl_169"}
// Dependencies: {}
impl NotForContentType { # [doc = " Predicate that wont compress gRPC responses."] pub const GRPC : Self = Self :: const_new ("application/grpc") ; # [doc = " Predicate that wont compress images."] pub const IMAGES : Self = Self { content_type : Str :: Static ("image/") , exception : Some (Str :: Static ("image/svg+xml")) , } ; # [doc = " Predicate that wont compress Server-Sent Events (SSE) responses."] pub const SSE : Self = Self :: const_new ("text/event-stream") ; # [doc = " Create a new `NotForContentType`."] pub fn new (content_type : & str) -> Self { Self { content_type : Str :: Shared (content_type . into ()) , exception : None , } } # [doc = " Create a new `NotForContentType` from a static string."] pub const fn const_new (content_type : & 'static str) -> Self { Self { content_type : Str :: Static (content_type) , exception : None , } } }
};
}
