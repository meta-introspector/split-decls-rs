// Generated macro for contains_ignore_ascii_case (function)
macro_rules! Depcrate_compression_futurecontains_ignore_ascii_case {
() => {
// Module: crate::compression::future
// Provides: {"contains_ignore_ascii_case"}
// Dependencies: {}
fn contains_ignore_ascii_case (mut haystack : & [u8] , needle : & [u8]) -> bool { while needle . len () <= haystack . len () { if haystack [.. needle . len ()] . eq_ignore_ascii_case (needle) { return true ; } haystack = & haystack [1 ..] ; } false }
};
}
