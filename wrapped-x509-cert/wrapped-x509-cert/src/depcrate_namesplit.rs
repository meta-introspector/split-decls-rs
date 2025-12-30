// Generated macro for split (function)
macro_rules! Depcrate_namesplit {
() => {
// Module: crate::name
// Provides: {"split"}
// Dependencies: {}
# [doc = " Split a string at all non-escaped separators."] fn split (s : & str , b : u8) -> impl '_ + Iterator < Item = & '_ str > { let mut prev = 0 ; find (s , b) . chain ([s . len ()]) . map (move | i | { let x = & s [prev .. i] ; prev = i + 1 ; x }) }
};
}
