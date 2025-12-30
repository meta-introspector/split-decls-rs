// Generated macro for impl_699 (impl)
macro_rules! Depcrate_unix_freebsd_systemimpl_699 {
() => {
// Module: crate::unix::freebsd::system
// Provides: {"impl_699"}
// Dependencies: {}
impl Drop for SystemInfo { fn drop (& mut self) { unsafe { libc :: kvm_close (self . kd . as_ptr ()) ; if ! self . procstat . is_null () { libc :: procstat_close (self . procstat) ; } } } }
};
}
