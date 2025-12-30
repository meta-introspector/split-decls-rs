// Generated macro for impl_141 (impl)
macro_rules! Depcrate_hash_mapimpl_141 {
() => {
// Module: crate::hash_map
// Provides: {"impl_141"}
// Dependencies: {}
impl < K , V , H > PartialEq for HashMap < K , V , H > where K : Eq + Hash , V : PartialEq , H : BuildHasher , { # [doc = " Compares two [`HashMap`] instances."] # [doc = ""] # [doc = " # Locking behavior"] # [doc = ""] # [doc = " Shared locks on buckets are acquired when comparing two instances of [`HashMap`], therefore"] # [doc = " this may lead to deadlocks if the instances are being modified by another thread."] # [inline] fn eq (& self , other : & Self) -> bool { if self . iter_sync (| k , v | other . read_sync (k , | _ , ov | v == ov) == Some (true)) { return other . iter_sync (| k , v | self . read_sync (k , | _ , sv | v == sv) == Some (true)) ; } false } }
};
}
