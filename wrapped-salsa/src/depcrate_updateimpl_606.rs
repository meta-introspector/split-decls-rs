// Generated macro for impl_606 (impl)
macro_rules! Depcrate_updateimpl_606 {
() => {
// Module: crate::update
// Provides: {"impl_606"}
// Dependencies: {}
unsafe impl < K , V , S > Update for indexmap :: IndexMap < K , V , S > where K : Update + Eq + Hash , V : Update , S : BuildHasher , { unsafe fn maybe_update (old_pointer : * mut Self , new_map : Self) -> bool { maybe_update_map ! (old_pointer , new_map) } }
};
}
