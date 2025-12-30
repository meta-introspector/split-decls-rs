// Generated macro for impl_663 (impl)
macro_rules! Depcrate_unix_freebsd_processimpl_663 {
() => {
// Module: crate::unix::freebsd::process
// Provides: {"impl_663"}
// Dependencies: {}
impl < T > AllocatedPtr < T > { fn new (size : libc :: size_t) -> Option < Self > { unsafe { let ptr = libc :: malloc (size) ; if ptr . is_null () { None } else { Some (Self (ptr as _)) } } } }
};
}
