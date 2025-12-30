// Generated macro for count_lf_crlf (function)
macro_rules! Depcrate_utilscount_lf_crlf {
() => {
// Module: crate::utils
// Provides: {"count_lf_crlf"}
// Dependencies: {}
# [doc = " Returns the number of LF and CRLF respectively."] pub (crate) fn count_lf_crlf (input : & str) -> (usize , usize) { let mut lf = 0 ; let mut crlf = 0 ; let mut is_crlf = false ; for c in input . as_bytes () { match c { b'\r' => is_crlf = true , b'\n' if is_crlf => crlf += 1 , b'\n' => lf += 1 , _ => is_crlf = false , } } (lf , crlf) }
};
}
