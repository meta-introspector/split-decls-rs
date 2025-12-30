// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl Drop for TempFile { fn drop (& mut self) { if self . delete_on_drop { let result = Self :: remove_file (self . path_buf . as_path ()) ; if self . panic_on_delete_err { if let Err (e) = result { panic ! ("{}" , e) ; } } } } }
};
}
