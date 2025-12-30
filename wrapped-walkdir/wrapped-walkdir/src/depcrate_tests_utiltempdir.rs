// Generated macro for TempDir (struct)
macro_rules! Depcrate_tests_utilTempDir {
() => {
// Module: crate::tests::util
// Provides: {"TempDir"}
// Dependencies: {}
# [doc = " A simple wrapper for creating a temporary directory that is automatically"] # [doc = " deleted when it's dropped."] # [doc = ""] # [doc = " We use this in lieu of tempfile because tempfile brings in too many"] # [doc = " dependencies."] # [derive (Debug)] pub struct TempDir (PathBuf) ;
};
}
