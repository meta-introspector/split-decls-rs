// Generated macro for impl_348 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_348 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_348"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < K , Q : ? Sized , V , S > Index < & Q > for HashMap < K , V , S > where K : Eq + Hash + Borrow < Q > , Q : Eq + Hash , S : BuildHasher , { type Output = V ; # [doc = " Returns a reference to the value corresponding to the supplied key."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the key is not present in the `HashMap`."] # [inline] fn index (& self , key : & Q) -> & V { self . get (key) . expect ("no entry found for key") } }
};
}
