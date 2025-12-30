// Generated macro for impl_42 (impl)
macro_rules! Depcrate_hash_cacheimpl_42 {
() => {
// Module: crate::hash_cache
// Provides: {"impl_42"}
// Dependencies: {}
impl < K , V , H > PartialEq for HashCache < K , V , H > where K : Eq + Hash , V : PartialEq , H : BuildHasher , { # [doc = " Compares two [`HashCache`] instances."] # [doc = ""] # [doc = " ## Locking behavior"] # [doc = ""] # [doc = " Shared locks on buckets are acquired when comparing two instances of [`HashCache`], therefore"] # [doc = " this may lead to a deadlock if the instances are being modified by another thread."] # [inline] fn eq (& self , other : & Self) -> bool { if self . iter_sync (| k , v | other . read_sync (k , | _ , ov | v == ov) == Some (true)) { return other . iter_sync (| k , v | self . read_sync (k , | _ , sv | v == sv) == Some (true)) ; } false } }
};
}
