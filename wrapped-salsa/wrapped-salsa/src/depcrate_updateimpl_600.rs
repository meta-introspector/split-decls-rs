// Generated macro for impl_600 (impl)
macro_rules! Depcrate_updateimpl_600 {
() => {
// Module: crate::update
// Provides: {"impl_600"}
// Dependencies: {}
unsafe impl < K , V , S > Update for HashMap < K , V , S > where K : Update + Eq + Hash , V : Update , S : BuildHasher , { unsafe fn maybe_update (old_pointer : * mut Self , new_map : Self) -> bool { maybe_update_map ! (old_pointer , new_map) } }
};
}
