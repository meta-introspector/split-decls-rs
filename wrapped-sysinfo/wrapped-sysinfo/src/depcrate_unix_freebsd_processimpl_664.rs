// Generated macro for impl_664 (impl)
macro_rules! Depcrate_unix_freebsd_processimpl_664 {
() => {
// Module: crate::unix::freebsd::process
// Provides: {"impl_664"}
// Dependencies: {}
impl < T > Drop for AllocatedPtr < T > { fn drop (& mut self) { if ! self . 0 . is_null () { unsafe { libc :: free (self . 0 as _) ; } } } }
};
}
