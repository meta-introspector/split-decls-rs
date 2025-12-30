// Generated macro for local_sort (function)
macro_rules! Depcrate_statslocal_sort {
() => {
// Module: crate::stats
// Provides: {"local_sort"}
// Dependencies: {}
fn local_sort (v : & mut [f64]) { v . sort_by (| x : & f64 , y : & f64 | x . total_cmp (y)) ; }
};
}
