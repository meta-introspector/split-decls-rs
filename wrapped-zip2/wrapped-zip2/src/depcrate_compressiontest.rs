// Generated macro for test (module)
macro_rules! Depcrate_compressiontest {
() => {
// Module: crate::compression
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: { CompressionMethod , SUPPORTED_COMPRESSION_METHODS } ; # [test] fn from_eq_to () { for v in 0 .. (u16 :: MAX as u32 + 1) { let from = CompressionMethod :: parse_from_u16 (v as u16) ; let to = from . serialize_to_u16 () as u32 ; assert_eq ! (v , to) ; } } # [test] fn to_eq_from () { fn check_match (method : CompressionMethod) { let to = method . serialize_to_u16 () ; let from = CompressionMethod :: parse_from_u16 (to) ; let back = from . serialize_to_u16 () ; assert_eq ! (to , back) ; } for & method in SUPPORTED_COMPRESSION_METHODS { check_match (method) ; } } # [test] fn to_display_fmt () { fn check_match (method : CompressionMethod) { let debug_str = format ! ("{method:?}") ; let display_str = format ! ("{method}") ; assert_eq ! (debug_str , display_str) ; } for & method in SUPPORTED_COMPRESSION_METHODS { check_match (method) ; } } }
};
}
