// Generated macro for impl_481 (impl)
macro_rules! Depcrate_collections_hash_setimpl_481 {
() => {
// Module: crate::collections::hash::set
// Provides: {"impl_481"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < K > Iterator for IntoIter < K > { type Item = K ; # [inline] fn next (& mut self) -> Option < K > { self . base . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . base . size_hint () } # [inline] fn count (self) -> usize { self . base . len () } # [inline] fn fold < B , F > (self , init : B , f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . base . fold (init , f) } }
};
}
