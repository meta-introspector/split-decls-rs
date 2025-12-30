// Generated macro for fallback_impl (macro)
macro_rules! Depcrate_updatefallback_impl {
() => {
// Module: crate::update
// Provides: {"fallback_impl"}
// Dependencies: {}
macro_rules ! fallback_impl { ($ ($ t : ty ,) *) => { $ (unsafe impl Update for $ t { unsafe fn maybe_update (old_pointer : * mut Self , new_value : Self) -> bool { unsafe { update_fallback (old_pointer , new_value) } } }) * } }
};
}
