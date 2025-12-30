// Generated macro for impl_593 (impl)
macro_rules! Depcrate_updateimpl_593 {
() => {
// Module: crate::update
// Provides: {"impl_593"}
// Dependencies: {}
unsafe impl < T > Update for Vec < T > where T : Update , { unsafe fn maybe_update (old_pointer : * mut Self , new_vec : Self) -> bool { maybe_update_vec ! (old_pointer , new_vec , T) } }
};
}
