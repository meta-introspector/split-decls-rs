// Generated macro for impl_182 (impl)
macro_rules! Depcrate_hash_setimpl_182 {
() => {
// Module: crate::hash_set
// Provides: {"impl_182"}
// Dependencies: {}
impl < K , H > PartialEq for HashSet < K , H > where K : Eq + Hash , H : BuildHasher , { # [doc = " Compares two [`HashSet`] instances."] # [doc = ""] # [doc = " ### Locking behavior"] # [doc = ""] # [doc = " Shared locks on buckets are acquired when comparing two instances of [`HashSet`], therefore"] # [doc = " it may lead to a deadlock if the instances are being modified by another thread."] # [inline] fn eq (& self , other : & Self) -> bool { if self . iter_sync (| k | other . contains_sync (k)) { return other . iter_sync (| k | self . contains_sync (k)) ; } false } }
};
}
