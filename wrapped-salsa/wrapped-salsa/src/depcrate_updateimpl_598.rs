// Generated macro for impl_598 (impl)
macro_rules! Depcrate_updateimpl_598 {
() => {
// Module: crate::update
// Provides: {"impl_598"}
// Dependencies: {}
unsafe impl < T , const N : usize > Update for smallvec :: SmallVec < T , N > where T : Update , { unsafe fn maybe_update (old_pointer : * mut Self , new_vec : Self) -> bool { maybe_update_vec ! (old_pointer , new_vec , T) } }
};
}
