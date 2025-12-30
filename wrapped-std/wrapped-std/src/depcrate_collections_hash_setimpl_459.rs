// Generated macro for impl_459 (impl)
macro_rules! Depcrate_collections_hash_setimpl_459 {
() => {
// Module: crate::collections::hash::set
// Provides: {"impl_459"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T , S > Default for HashSet < T , S > where S : Default , { # [doc = " Creates an empty `HashSet<T, S>` with the `Default` value for the hasher."] # [inline] fn default () -> HashSet < T , S > { HashSet { base : Default :: default () } } }
};
}
