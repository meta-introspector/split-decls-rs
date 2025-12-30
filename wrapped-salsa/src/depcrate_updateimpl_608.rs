// Generated macro for impl_608 (impl)
macro_rules! Depcrate_updateimpl_608 {
() => {
// Module: crate::update
// Provides: {"impl_608"}
// Dependencies: {}
unsafe impl < K , V > Update for BTreeMap < K , V > where K : Update + Eq + Ord , V : Update , { unsafe fn maybe_update (old_pointer : * mut Self , new_map : Self) -> bool { maybe_update_map ! (old_pointer , new_map) } }
};
}
