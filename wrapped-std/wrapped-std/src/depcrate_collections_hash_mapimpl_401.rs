// Generated macro for impl_401 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_401 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_401"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , K , V > Iterator for Keys < 'a , K , V > { type Item = & 'a K ; # [inline] fn next (& mut self) -> Option < & 'a K > { self . inner . next () . map (| (k , _) | k) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } # [inline] fn count (self) -> usize { self . inner . len () } # [inline] fn fold < B , F > (self , init : B , mut f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . inner . fold (init , | acc , (k , _) | f (acc , k)) } }
};
}
