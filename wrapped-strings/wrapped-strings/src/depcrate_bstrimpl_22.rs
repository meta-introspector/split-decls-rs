// Generated macro for impl_22 (impl)
macro_rules! Depcrate_bstrimpl_22 {
() => {
// Module: crate::bstr
// Provides: {"impl_22"}
// Dependencies: {}
impl Drop for BSTR { fn drop (& mut self) { if ! self . 0 . is_null () { unsafe { bindings :: SysFreeString (self . 0) } } } }
};
}
