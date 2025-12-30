// Generated macro for impl_84 (impl)
macro_rules! Depcrate_hstring_builderimpl_84 {
() => {
// Module: crate::hstring_builder
// Provides: {"impl_84"}
// Dependencies: {}
impl Drop for HStringBuilder { fn drop (& mut self) { unsafe { HStringHeader :: free (self . 0) ; } } }
};
}
