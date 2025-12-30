// Generated macro for impl_347 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_347 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_347"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < K , V , S > Default for HashMap < K , V , S > where S : Default , { # [doc = " Creates an empty `HashMap<K, V, S>`, with the `Default` value for the hasher."] # [inline] fn default () -> HashMap < K , V , S > { HashMap :: with_hasher (Default :: default ()) } }
};
}
