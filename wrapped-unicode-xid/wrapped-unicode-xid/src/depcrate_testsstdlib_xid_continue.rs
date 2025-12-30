// Generated macro for stdlib_xid_continue (function)
macro_rules! Depcrate_testsstdlib_xid_continue {
() => {
// Module: crate::tests
// Provides: {"stdlib_xid_continue"}
// Dependencies: {}
# [cfg (feature = "bench")] # [bench] fn stdlib_xid_continue (b : & mut Bencher) { let string = iter :: repeat ('a') . take (4096) . collect :: < String > () ; b . bytes = string . len () as u64 ; b . iter (| | string . chars () . all (char :: is_xid_continue)) ; }
};
}
