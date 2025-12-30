// Generated macro for impl_39 (impl)
macro_rules! Depcrate_hash_cacheimpl_39 {
() => {
// Module: crate::hash_cache
// Provides: {"impl_39"}
// Dependencies: {}
impl < K , V , H > Drop for HashCache < K , V , H > where H : BuildHasher , { # [inline] fn drop (& mut self) { self . bucket_array . swap ((None , Tag :: None) , Relaxed) . 0 . map (| a | unsafe { a . drop_in_place () }) ; } }
};
}
