// Generated macro for impl_465 (impl)
macro_rules! Depcrate_copy_dir_recursiveimpl_465 {
() => {
// Module: crate::copy_dir_recursive
// Provides: {"impl_465"}
// Dependencies: {}
impl Drop for LockFileGuard { fn drop (& mut self) { if let Err (e) = fs :: remove_file (& self . path) { eprintln ! ("Warning: Failed to remove lock file at {}: {}" , self . path . display () , e) ; } } }
};
}
