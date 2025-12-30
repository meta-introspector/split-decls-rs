// Generated macro for impl_604 (impl)
macro_rules! Depcrate_updateimpl_604 {
() => {
// Module: crate::update
// Provides: {"impl_604"}
// Dependencies: {}
unsafe impl < K , S > Update for indexmap :: IndexSet < K , S > where K : Update + Eq + Hash , S : BuildHasher , { unsafe fn maybe_update (old_pointer : * mut Self , new_set : Self) -> bool { maybe_update_set ! (old_pointer , new_set) } }
};
}
