// Generated macro for impl_23 (impl)
macro_rules! Depcrate_dirimpl_23 {
() => {
// Module: crate::dir
// Provides: {"impl_23"}
// Dependencies: {}
impl Drop for TempDir { fn drop (& mut self) { if ! self . disable_cleanup { let _ = remove_dir_all (self . path ()) ; } } }
};
}
