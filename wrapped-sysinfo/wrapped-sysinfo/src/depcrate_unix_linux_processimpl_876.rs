// Generated macro for impl_876 (impl)
macro_rules! Depcrate_unix_linux_processimpl_876 {
() => {
// Module: crate::unix::linux::process
// Provides: {"impl_876"}
// Dependencies: {}
impl Drop for FileCounter { fn drop (& mut self) { remaining_files () . fetch_add (1 , Ordering :: Relaxed) ; } }
};
}
