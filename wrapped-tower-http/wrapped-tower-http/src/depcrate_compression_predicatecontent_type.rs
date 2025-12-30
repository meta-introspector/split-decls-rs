// Generated macro for content_type (function)
macro_rules! Depcrate_compression_predicatecontent_type {
() => {
// Module: crate::compression::predicate
// Provides: {"content_type"}
// Dependencies: {}
fn content_type < B > (response : & http :: Response < B >) -> & str { response . headers () . get (header :: CONTENT_TYPE) . and_then (| h | h . to_str () . ok ()) . unwrap_or_default () }
};
}
