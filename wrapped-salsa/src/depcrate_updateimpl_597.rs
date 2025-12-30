// Generated macro for impl_597 (impl)
macro_rules! Depcrate_updateimpl_597 {
() => {
// Module: crate::update
// Provides: {"impl_597"}
// Dependencies: {}
unsafe impl < T > Update for thin_vec :: ThinVec < T > where T : Update , { unsafe fn maybe_update (old_pointer : * mut Self , new_vec : Self) -> bool { maybe_update_vec ! (old_pointer , new_vec , T) } }
};
}
