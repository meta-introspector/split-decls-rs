// Generated macro for impl_1298 (impl)
macro_rules! Depcrate_windows_systemimpl_1298 {
() => {
// Module: crate::windows::system
// Provides: {"impl_1298"}
// Dependencies: {}
impl Drop for RegKey { fn drop (& mut self) { let _err = unsafe { RegCloseKey (self . 0) } ; } }
};
}
