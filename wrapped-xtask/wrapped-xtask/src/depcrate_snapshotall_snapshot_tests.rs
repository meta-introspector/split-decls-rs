// Generated macro for all_snapshot_tests (function)
macro_rules! Depcrate_snapshotall_snapshot_tests {
() => {
// Module: crate::snapshot
// Provides: {"all_snapshot_tests"}
// Dependencies: {}
pub (crate) fn all_snapshot_tests () -> Vec < & 'static str > { pub const STABLE_SNAPSHOT_TESTS : & [& str] = & ["log" , "bitflags" , "timestamp" , "panic" , "assert" , "assert-eq" , "assert-ne" , "unwrap" , "defmt-test" , "hints" , "hints_inner" , "dbg" , "panic_info" ,] ; const NIGHTLY_SNAPSHOT_TESTS : & [& str] = & ["alloc"] ; const POST_MSRV_SNAPSHOT_TESTS : & [& str] = & ["net"] ; let mut tests = STABLE_SNAPSHOT_TESTS . to_vec () ; if rustc_is_nightly () { tests . extend (NIGHTLY_SNAPSHOT_TESTS) ; } if ! rustc_is_msrv () { tests . extend (POST_MSRV_SNAPSHOT_TESTS) ; } tests }
};
}
