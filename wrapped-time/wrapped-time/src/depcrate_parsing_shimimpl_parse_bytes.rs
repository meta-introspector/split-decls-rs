// Generated macro for impl_parse_bytes (macro)
macro_rules! Depcrate_parsing_shimimpl_parse_bytes {
() => {
// Module: crate::parsing::shim
// Provides: {"impl_parse_bytes"}
// Dependencies: {}
# [doc = " Parse the given types from bytes."] macro_rules ! impl_parse_bytes { ($ ($ t : ty) *) => ($ (impl Integer for $ t { # [allow (trivial_numeric_casts)] # [inline] fn parse_bytes (src : & [u8]) -> Option < Self > { src . iter () . try_fold ::< Self , _ , _ > (0 , | result , c | { result . checked_mul (10) ?. checked_add ((c - b'0') as Self) }) } }) *) }
};
}
