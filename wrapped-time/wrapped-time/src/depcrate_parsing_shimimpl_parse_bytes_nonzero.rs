// Generated macro for impl_parse_bytes_nonzero (macro)
macro_rules! Depcrate_parsing_shimimpl_parse_bytes_nonzero {
() => {
// Module: crate::parsing::shim
// Provides: {"impl_parse_bytes_nonzero"}
// Dependencies: {}
# [doc = " Parse the given types from bytes."] macro_rules ! impl_parse_bytes_nonzero { ($ ($ t : ty) *) => { $ (impl Integer for $ t { # [inline] fn parse_bytes (src : & [u8]) -> Option < Self > { Self :: new (src . parse_bytes () ?) } }) * } }
};
}
