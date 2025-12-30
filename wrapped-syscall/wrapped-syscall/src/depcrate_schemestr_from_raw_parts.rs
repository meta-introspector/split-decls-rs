// Generated macro for str_from_raw_parts (function)
macro_rules! Depcrate_schemestr_from_raw_parts {
() => {
// Module: crate::scheme
// Provides: {"str_from_raw_parts"}
// Dependencies: {}
unsafe fn str_from_raw_parts (ptr : * const u8 , len : usize) -> Option < & 'static str > { let slice = slice :: from_raw_parts (ptr , len) ; str :: from_utf8 (slice) . ok () }
};
}
