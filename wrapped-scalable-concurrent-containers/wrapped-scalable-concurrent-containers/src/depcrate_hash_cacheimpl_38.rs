// Generated macro for impl_38 (impl)
macro_rules! Depcrate_hash_cacheimpl_38 {
() => {
// Module: crate::hash_cache
// Provides: {"impl_38"}
// Dependencies: {}
impl < K , V , H > Debug for HashCache < K , V , H > where K : Debug + Eq + Hash , V : Debug , H : BuildHasher , { # [doc = " Iterates over all the entries in the [`HashCache`] to print them."] # [doc = ""] # [doc = " ## Locking behavior"] # [doc = ""] # [doc = " Shared locks on buckets are acquired during iteration, therefore any [`Entry`],"] # [doc = " [`OccupiedEntry`], or [`VacantEntry`] owned by the current thread will lead to a deadlock."] # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut d = f . debug_map () ; self . iter_sync (| k , v | { d . entry (k , v) ; true }) ; d . finish () } }
};
}
