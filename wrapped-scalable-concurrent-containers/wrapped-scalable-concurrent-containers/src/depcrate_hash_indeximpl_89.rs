// Generated macro for impl_89 (impl)
macro_rules! Depcrate_hash_indeximpl_89 {
() => {
// Module: crate::hash_index
// Provides: {"impl_89"}
// Dependencies: {}
impl < K , V , H > PartialEq for HashIndex < K , V , H > where K : Eq + Hash , V : PartialEq , H : BuildHasher , { # [inline] fn eq (& self , other : & Self) -> bool { let guard = Guard :: new () ; self . reclaim_memory (& guard) ; if ! self . iter (& guard) . any (| (k , v) | other . peek_with (k , | _ , ov | v == ov) != Some (true)) { return ! other . iter (& guard) . any (| (k , v) | self . peek_with (k , | _ , sv | v == sv) != Some (true)) ; } false } }
};
}
