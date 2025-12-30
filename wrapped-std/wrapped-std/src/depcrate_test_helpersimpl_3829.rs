// Generated macro for impl_3829 (impl)
macro_rules! Depcrate_test_helpersimpl_3829 {
() => {
// Module: crate::test_helpers
// Provides: {"impl_3829"}
// Dependencies: {}
impl Drop for TempDir { fn drop (& mut self) { let TempDir (ref p) = * self ; let result = fs :: remove_dir_all (p) ; if ! thread :: panicking () { result . unwrap () ; } } }
};
}
