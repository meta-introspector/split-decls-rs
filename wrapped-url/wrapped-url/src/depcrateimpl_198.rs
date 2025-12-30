// Generated macro for impl_198 (impl)
macro_rules! Depcrateimpl_198 {
() => {
// Module: crate
// Provides: {"impl_198"}
// Dependencies: {}
impl Drop for UrlQuery < '_ > { fn drop (& mut self) { if let Some (url) = self . url . take () { url . restore_already_parsed_fragment (self . fragment . take ()) } } }
};
}
