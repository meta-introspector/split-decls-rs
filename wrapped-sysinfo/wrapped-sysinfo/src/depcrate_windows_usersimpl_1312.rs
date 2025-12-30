// Generated macro for impl_1312 (impl)
macro_rules! Depcrate_windows_usersimpl_1312 {
() => {
// Module: crate::windows::users
// Provides: {"impl_1312"}
// Dependencies: {}
impl < T > Drop for NetApiBuffer < T > { fn drop (& mut self) { if ! self . 0 . is_null () { unsafe { NetApiBufferFree (Some (self . 0 . cast ())) } ; } } }
};
}
