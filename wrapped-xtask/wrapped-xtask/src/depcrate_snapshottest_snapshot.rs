// Generated macro for test_snapshot (function)
macro_rules! Depcrate_snapshottest_snapshot {
() => {
// Module: crate::snapshot
// Provides: {"test_snapshot"}
// Dependencies: {}
pub fn test_snapshot (overwrite : bool , snapshot : Option < Snapshot >) { println ! ("🧪 qemu/snapshot") ; match snapshot { None => test_all_snapshots (overwrite) , Some (snapshot) => { do_test (| | test_single_snapshot (snapshot . name () , "" , overwrite) , "qemu/snapshot" ,) ; } } }
};
}
