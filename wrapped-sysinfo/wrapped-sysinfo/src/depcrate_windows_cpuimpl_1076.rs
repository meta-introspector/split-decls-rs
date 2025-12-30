// Generated macro for impl_1076 (impl)
macro_rules! Depcrate_windows_cpuimpl_1076 {
() => {
// Module: crate::windows::cpu
// Provides: {"impl_1076"}
// Dependencies: {}
impl Drop for InternalQuery { fn drop (& mut self) { unsafe { for (_ , counter) in self . data . iter () { PdhRemoveCounter (* counter) ; } if ! self . event . is_invalid () { let _err = CloseHandle (self . event) ; } if ! self . query . is_invalid () { PdhCloseQuery (self . query) ; } } } }
};
}
