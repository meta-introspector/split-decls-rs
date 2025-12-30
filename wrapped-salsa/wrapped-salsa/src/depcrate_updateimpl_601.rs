// Generated macro for impl_601 (impl)
macro_rules! Depcrate_updateimpl_601 {
() => {
// Module: crate::update
// Provides: {"impl_601"}
// Dependencies: {}
unsafe impl < K > Update for BTreeSet < K > where K : Update + Eq + Ord , { unsafe fn maybe_update (old_pointer : * mut Self , new_set : Self) -> bool { maybe_update_set ! (old_pointer , new_set) } }
};
}
