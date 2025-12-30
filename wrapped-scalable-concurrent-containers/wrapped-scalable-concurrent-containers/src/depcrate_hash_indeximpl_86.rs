// Generated macro for impl_86 (impl)
macro_rules! Depcrate_hash_indeximpl_86 {
() => {
// Module: crate::hash_index
// Provides: {"impl_86"}
// Dependencies: {}
impl < K , V , H > Drop for HashIndex < K , V , H > where H : BuildHasher , { # [inline] fn drop (& mut self) { self . bucket_array . swap ((None , Tag :: None) , Relaxed) . 0 . map (| a | unsafe { a . drop_in_place () }) ; let mut garbage_head = self . garbage_chain . swap ((None , Tag :: None) , Acquire) . 0 ; while let Some (garbage_bucket_array) = garbage_head { garbage_head = garbage_bucket_array . bucket_link () . swap ((None , Tag :: None) , Acquire) . 0 ; let dropped = unsafe { garbage_bucket_array . drop_in_place () } ; debug_assert ! (dropped) ; } } }
};
}
