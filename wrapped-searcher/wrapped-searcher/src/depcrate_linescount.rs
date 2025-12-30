// Generated macro for count (function)
macro_rules! Depcrate_linescount {
() => {
// Module: crate::lines
// Provides: {"count"}
// Dependencies: {}
# [doc = " Count the number of occurrences of `line_term` in `bytes`."] pub (crate) fn count (bytes : & [u8] , line_term : u8) -> u64 { memchr :: memchr_iter (line_term , bytes) . count () as u64 }
};
}
