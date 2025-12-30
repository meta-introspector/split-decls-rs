// Generated macro for impl_393 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_393 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_393"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , K , V > Iterator for IterMut < 'a , K , V > { type Item = (& 'a K , & 'a mut V) ; # [inline] fn next (& mut self) -> Option < (& 'a K , & 'a mut V) > { self . base . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . base . size_hint () } # [inline] fn count (self) -> usize { self . base . len () } # [inline] fn fold < B , F > (self , init : B , f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . base . fold (init , f) } }
};
}
