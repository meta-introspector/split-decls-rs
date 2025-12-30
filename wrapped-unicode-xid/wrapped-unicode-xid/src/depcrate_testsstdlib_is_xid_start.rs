// Generated macro for stdlib_is_xid_start (function)
macro_rules! Depcrate_testsstdlib_is_xid_start {
() => {
// Module: crate::tests
// Provides: {"stdlib_is_xid_start"}
// Dependencies: {}
# [cfg (feature = "bench")] # [bench] fn stdlib_is_xid_start (b : & mut Bencher) { let string = iter :: repeat ('a') . take (4096) . collect :: < String > () ; b . bytes = string . len () as u64 ; b . iter (| | string . chars () . all (char :: is_xid_start)) ; }
};
}
