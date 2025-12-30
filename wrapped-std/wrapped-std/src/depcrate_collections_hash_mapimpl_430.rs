// Generated macro for impl_430 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_430 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_430"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < K , V , S > FromIterator < (K , V) > for HashMap < K , V , S > where K : Eq + Hash , S : BuildHasher + Default , { # [doc = " Constructs a `HashMap<K, V>` from an iterator of key-value pairs."] # [doc = ""] # [doc = " If the iterator produces any pairs with equal keys,"] # [doc = " all but one of the corresponding values will be dropped."] fn from_iter < T : IntoIterator < Item = (K , V) > > (iter : T) -> HashMap < K , V , S > { let mut map = HashMap :: with_hasher (Default :: default ()) ; map . extend (iter) ; map } }
};
}
