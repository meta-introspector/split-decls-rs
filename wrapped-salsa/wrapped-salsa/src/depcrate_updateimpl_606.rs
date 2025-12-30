// Generated macro for impl_606 (impl)
macro_rules! Depcrate_updateimpl_606 {
() => {
// Module: crate::update
// Provides: {"impl_606"}
// Dependencies: {}
unsafe impl < T > Update for Box < T > where T : Update , { unsafe fn maybe_update (old_pointer : * mut Self , new_box : Self) -> bool { let old_box : & mut Box < T > = unsafe { & mut * old_pointer } ; unsafe { T :: maybe_update (& mut * * old_box , * new_box) } } }
};
}
