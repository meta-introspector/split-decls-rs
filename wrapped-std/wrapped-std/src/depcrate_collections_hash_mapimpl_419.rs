// Generated macro for impl_419 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_419 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_419"}
// Dependencies: {}
# [stable (feature = "drain" , since = "1.6.0")] impl < 'a , K , V > Iterator for Drain < 'a , K , V > { type Item = (K , V) ; # [inline] fn next (& mut self) -> Option < (K , V) > { self . base . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . base . size_hint () } # [inline] fn fold < B , F > (self , init : B , f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . base . fold (init , f) } }
};
}
