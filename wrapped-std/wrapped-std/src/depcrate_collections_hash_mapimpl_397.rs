// Generated macro for impl_397 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_397 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_397"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < K , V > Iterator for IntoIter < K , V > { type Item = (K , V) ; # [inline] fn next (& mut self) -> Option < (K , V) > { self . base . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . base . size_hint () } # [inline] fn count (self) -> usize { self . base . len () } # [inline] fn fold < B , F > (self , init : B , f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . base . fold (init , f) } }
};
}
