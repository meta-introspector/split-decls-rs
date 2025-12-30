// Generated macro for impl_103 (impl)
macro_rules! Depcrate_hash_indeximpl_103 {
() => {
// Module: crate::hash_index
// Provides: {"impl_103"}
// Dependencies: {}
impl < K , V , H > Drop for Reserve < '_ , K , V , H > where K : Eq + Hash , H : BuildHasher , { # [inline] fn drop (& mut self) { let result = self . hashindex . minimum_capacity . fetch_sub (self . additional , Relaxed) ; debug_assert ! (result >= self . additional) ; let guard = Guard :: new () ; if let Some (current_array) = self . hashindex . bucket_array . load (Acquire , & guard) . as_ref () { self . try_shrink_or_rebuild (current_array , 0 , & guard) ; } } }
};
}
