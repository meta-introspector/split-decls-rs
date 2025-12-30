// Generated macro for impl_26 (impl)
macro_rules! Depcrate_snapshotimpl_26 {
() => {
// Module: crate::snapshot
// Provides: {"impl_26"}
// Dependencies: {}
impl FromStr for Snapshot { type Err = String ; fn from_str (test : & str) -> Result < Self , Self :: Err > { let all_tests = all_snapshot_tests () ; if all_tests . contains (& test) { Ok (Self (String :: from (test))) } else { Err (format ! ("Specified test '{test}' does not exist, available tests are: {all_tests:?}")) } } }
};
}
