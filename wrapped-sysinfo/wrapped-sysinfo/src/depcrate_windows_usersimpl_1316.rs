// Generated macro for impl_1316 (impl)
macro_rules! Depcrate_windows_usersimpl_1316 {
() => {
// Module: crate::windows::users
// Provides: {"impl_1316"}
// Dependencies: {}
impl < T > Drop for LsaBuffer < T > { fn drop (& mut self) { if ! self . 0 . is_null () { let _r = unsafe { LsaFreeReturnBuffer (self . 0 as * mut _) } ; } } }
};
}
