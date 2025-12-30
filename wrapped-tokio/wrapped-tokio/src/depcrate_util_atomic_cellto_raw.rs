// Generated macro for to_raw (function)
macro_rules! Depcrate_util_atomic_cellto_raw {
() => {
// Module: crate::util::atomic_cell
// Provides: {"to_raw"}
// Dependencies: {}
fn to_raw < T > (data : Option < Box < T > >) -> * mut T { data . map_or (ptr :: null_mut () , Box :: into_raw) }
};
}
