// Generated macro for test_all_snapshots (function)
macro_rules! Depcrate_snapshottest_all_snapshots {
() => {
// Module: crate::snapshot
// Provides: {"test_all_snapshots"}
// Dependencies: {}
fn test_all_snapshots (overwrite : bool) { for test in all_snapshot_tests () { let features = match test { "alloc" => "alloc" , "net" => "ip_in_core" , _ => "" , } ; do_test (| | test_single_snapshot (test , features , overwrite) , "qemu/snapshot" ,) ; } }
};
}
