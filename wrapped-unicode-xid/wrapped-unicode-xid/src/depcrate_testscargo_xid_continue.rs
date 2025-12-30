// Generated macro for cargo_xid_continue (function)
macro_rules! Depcrate_testscargo_xid_continue {
() => {
// Module: crate::tests
// Provides: {"cargo_xid_continue"}
// Dependencies: {}
# [cfg (feature = "bench")] # [bench] fn cargo_xid_continue (b : & mut Bencher) { let string = iter :: repeat ('a') . take (4096) . collect :: < String > () ; b . bytes = string . len () as u64 ; b . iter (| | string . chars () . all (super :: UnicodeXID :: is_xid_continue)) ; }
};
}
