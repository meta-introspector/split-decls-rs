// Generated macro for inline_as_str (function)
macro_rules! Depcrate_identifierinline_as_str {
() => {
// Module: crate::identifier
// Provides: {"inline_as_str"}
// Dependencies: {}
unsafe fn inline_as_str (repr : & Identifier) -> & str { let ptr = repr as * const Identifier as * const u8 ; let len = unsafe { inline_len (repr) } . get () ; let slice = unsafe { slice :: from_raw_parts (ptr , len) } ; unsafe { str :: from_utf8_unchecked (slice) } }
};
}
