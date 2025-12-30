// Generated macro for impl_477 (impl)
macro_rules! Depcrate_collections_hash_setimpl_477 {
() => {
// Module: crate::collections::hash::set
// Provides: {"impl_477"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , K > Iterator for Iter < 'a , K > { type Item = & 'a K ; # [inline] fn next (& mut self) -> Option < & 'a K > { self . base . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . base . size_hint () } # [inline] fn count (self) -> usize { self . base . len () } # [inline] fn fold < B , F > (self , init : B , f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . base . fold (init , f) } }
};
}
