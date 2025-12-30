// Generated macro for is_match_ns (function)
macro_rules! Depcrate_syn_utilsis_match_ns {
() => {
// Module: crate::syn_utils
// Provides: {"is_match_ns"}
// Dependencies: {}
pub fn is_match_ns (ss : & Punctuated < PathSegment , Token ! [::] > , ns : & [& str]) -> bool { let mut i_ss = ss . len () - 1 ; let mut i_ns = ns . len () ; while i_ss > 0 && i_ns > 0 { i_ns -= 1 ; i_ss -= 1 ; let s = & ss [i_ss] ; if s . ident != ns [i_ns] || ! s . arguments . is_empty () { return false ; } } i_ss == 0 }
};
}
