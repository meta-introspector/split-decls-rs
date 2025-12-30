// Generated macro for DefaultPredicate (struct)
macro_rules! Depcrate_compression_predicateDefaultPredicate {
() => {
// Module: crate::compression::predicate
// Provides: {"DefaultPredicate"}
// Dependencies: {}
# [doc = " The default predicate used by [`Compression`] and [`CompressionLayer`]."] # [doc = ""] # [doc = " This will compress responses unless:"] # [doc = ""] # [doc = " - They're gRPC, which has its own protocol specific compression scheme."] # [doc = " - It's an image as determined by the `content-type` starting with `image/`."] # [doc = " - They're Server-Sent Events (SSE) as determined by the `content-type` being `text/event-stream`."] # [doc = " - The response is less than 32 bytes."] # [doc = ""] # [doc = " # Configuring the defaults"] # [doc = ""] # [doc = " `DefaultPredicate` doesn't support any configuration. Instead you can build your own predicate"] # [doc = " by combining types in this module:"] # [doc = ""] # [doc = " ```rust"] # [doc = " use tower_http::compression::predicate::{SizeAbove, NotForContentType, Predicate};"] # [doc = ""] # [doc = " // slightly large min size than the default 32"] # [doc = " let predicate = SizeAbove::new(256)"] # [doc = "     // still don't compress gRPC"] # [doc = "     .and(NotForContentType::GRPC)"] # [doc = "     // still don't compress images"] # [doc = "     .and(NotForContentType::IMAGES)"] # [doc = "     // also don't compress JSON"] # [doc = "     .and(NotForContentType::const_new(\"application/json\"));"] # [doc = " ```"] # [doc = ""] # [doc = " [`Compression`]: super::Compression"] # [doc = " [`CompressionLayer`]: super::CompressionLayer"] # [derive (Clone)] pub struct DefaultPredicate (And < And < And < SizeAbove , NotForContentType > , NotForContentType > , NotForContentType > ,) ;
};
}
