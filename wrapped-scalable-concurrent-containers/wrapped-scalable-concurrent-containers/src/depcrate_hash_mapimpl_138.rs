// Generated macro for impl_138 (impl)
macro_rules! Depcrate_hash_mapimpl_138 {
() => {
// Module: crate::hash_map
// Provides: {"impl_138"}
// Dependencies: {}
impl < K , V , H > Drop for HashMap < K , V , H > where H : BuildHasher , { # [inline] fn drop (& mut self) { self . bucket_array . swap ((None , Tag :: None) , Relaxed) . 0 . map (| a | unsafe { a . drop_in_place () }) ; } }
};
}
