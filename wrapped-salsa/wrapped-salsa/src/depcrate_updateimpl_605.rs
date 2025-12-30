// Generated macro for impl_605 (impl)
macro_rules! Depcrate_updateimpl_605 {
() => {
// Module: crate::update
// Provides: {"impl_605"}
// Dependencies: {}
unsafe impl < K , V > Update for BTreeMap < K , V > where K : Update + Eq + Ord , V : Update , { unsafe fn maybe_update (old_pointer : * mut Self , new_map : Self) -> bool { maybe_update_map ! (old_pointer , new_map) } }
};
}
