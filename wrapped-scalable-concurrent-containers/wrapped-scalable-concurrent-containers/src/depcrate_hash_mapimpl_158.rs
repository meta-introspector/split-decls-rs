// Generated macro for impl_158 (impl)
macro_rules! Depcrate_hash_mapimpl_158 {
() => {
// Module: crate::hash_map
// Provides: {"impl_158"}
// Dependencies: {}
impl < K , V , H > Drop for Reserve < '_ , K , V , H > where K : Eq + Hash , H : BuildHasher , { # [inline] fn drop (& mut self) { let result = self . hashmap . minimum_capacity . fetch_sub (self . additional , Relaxed) ; debug_assert ! (result >= self . additional) ; let guard = Guard :: new () ; if let Some (current_array) = self . hashmap . bucket_array . load (Acquire , & guard) . as_ref () { self . try_shrink_or_rebuild (current_array , 0 , & guard) ; } } }
};
}
