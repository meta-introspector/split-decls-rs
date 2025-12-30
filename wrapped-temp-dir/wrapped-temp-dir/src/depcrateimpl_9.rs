// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
impl Drop for TempDir { fn drop (& mut self) { if self . delete_on_drop { let result = Self :: remove_dir (& self . path_buf) ; if self . panic_on_delete_err { if let Err (e) = result { panic ! ("{}" , e) ; } } } } }
};
}
