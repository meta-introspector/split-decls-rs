// Generated macro for impl_1049 (impl)
macro_rules! Depcrate_windows_componentimpl_1049 {
() => {
// Module: crate::windows::component
// Provides: {"impl_1049"}
// Dependencies: {}
impl Drop for Connection { fn drop (& mut self) { self . enumerator . take () ; self . server_connection . take () ; self . instance . take () ; } }
};
}
