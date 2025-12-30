// Generated macro for impl_136 (impl)
macro_rules! Depcrate_hash_mapimpl_136 {
() => {
// Module: crate::hash_map
// Provides: {"impl_136"}
// Dependencies: {}
impl < K , V , H > Debug for HashMap < K , V , H > where K : Debug + Eq + Hash , V : Debug , H : BuildHasher , { # [doc = " Iterates over all the entries in the [`HashMap`] to print them."] # [doc = ""] # [doc = " # Locking behavior"] # [doc = ""] # [doc = " Shared locks on buckets are acquired during iteration, therefore any [`Entry`],"] # [doc = " [`OccupiedEntry`], or [`VacantEntry`] owned by the current thread will lead to deadlocks."] # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut d = f . debug_map () ; self . iter_sync (| k , v | { d . entry (k , v) ; true }) ; d . finish () } }
};
}
