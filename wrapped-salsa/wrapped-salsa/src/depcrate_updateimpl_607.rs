// Generated macro for impl_607 (impl)
macro_rules! Depcrate_updateimpl_607 {
() => {
// Module: crate::update
// Provides: {"impl_607"}
// Dependencies: {}
unsafe impl < K , S > Update for indexmap :: IndexSet < K , S > where K : Update + Eq + Hash , S : BuildHasher , { unsafe fn maybe_update (old_pointer : * mut Self , new_set : Self) -> bool { maybe_update_set ! (old_pointer , new_set) } }
};
}
