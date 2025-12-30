// Generated macro for NotForContentType (struct)
macro_rules! Depcrate_compression_predicateNotForContentType {
() => {
// Module: crate::compression::predicate
// Provides: {"NotForContentType"}
// Dependencies: {}
# [doc = " Predicate that wont allow responses with a specific `content-type` to be compressed."] # [derive (Clone , Debug)] pub struct NotForContentType { content_type : Str , exception : Option < Str > , }
};
}
