// Generated macro for impl_600 (impl)
macro_rules! Depcrate_updateimpl_600 {
() => {
// Module: crate::update
// Provides: {"impl_600"}
// Dependencies: {}
unsafe impl < K , S > Update for HashSet < K , S > where K : Update + Eq + Hash , S : BuildHasher , { unsafe fn maybe_update (old_pointer : * mut Self , new_set : Self) -> bool { maybe_update_set ! (old_pointer , new_set) } }
};
}
