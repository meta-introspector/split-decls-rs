// Generated macro for impl_77 (impl)
macro_rules! Depcrate_fileimpl_77 {
() => {
// Module: crate::file
// Provides: {"impl_77"}
// Dependencies: {}
impl Drop for TempPath { fn drop (& mut self) { if ! self . disable_cleanup { let _ = fs :: remove_file (& self . path) ; } } }
};
}
