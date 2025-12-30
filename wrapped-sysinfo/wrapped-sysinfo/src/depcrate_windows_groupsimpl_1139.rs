// Generated macro for impl_1139 (impl)
macro_rules! Depcrate_windows_groupsimpl_1139 {
() => {
// Module: crate::windows::groups
// Provides: {"impl_1139"}
// Dependencies: {}
impl Drop for NetApiBuffer { fn drop (& mut self) { if ! self . 0 . is_null () { unsafe { NetApiBufferFree (Some (self . 0 . cast ())) } ; } } }
};
}
