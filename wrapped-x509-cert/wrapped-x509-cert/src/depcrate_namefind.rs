// Generated macro for find (function)
macro_rules! Depcrate_namefind {
() => {
// Module: crate::name
// Provides: {"find"}
// Dependencies: {}
# [doc = " Find the indices of all non-escaped separators."] fn find (s : & str , b : u8) -> impl '_ + Iterator < Item = usize > { (0 .. s . len ()) . filter (move | i | s . as_bytes () [* i] == b) . filter (| i | { let x = i . checked_sub (2) . map (| i | s . as_bytes () [i]) . unwrap_or_default () ; let y = i . checked_sub (1) . map (| i | s . as_bytes () [i]) . unwrap_or_default () ; y != b'\\' || x == b'\\' }) }
};
}
