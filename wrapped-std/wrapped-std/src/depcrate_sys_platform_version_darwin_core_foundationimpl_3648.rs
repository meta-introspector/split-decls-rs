// Generated macro for impl_3648 (impl)
macro_rules! Depcrate_sys_platform_version_darwin_core_foundationimpl_3648 {
() => {
// Module: crate::sys::platform_version::darwin::core_foundation
// Provides: {"impl_3648"}
// Dependencies: {}
impl Drop for CFHandle { fn drop (& mut self) { let _ = unsafe { libc :: dlclose (self . 0) } ; } }
};
}
