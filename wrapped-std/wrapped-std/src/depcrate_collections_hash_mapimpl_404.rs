// Generated macro for impl_404 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_404 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_404"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , K , V > Iterator for Values < 'a , K , V > { type Item = & 'a V ; # [inline] fn next (& mut self) -> Option < & 'a V > { self . inner . next () . map (| (_ , v) | v) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } # [inline] fn count (self) -> usize { self . inner . len () } # [inline] fn fold < B , F > (self , init : B , mut f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . inner . fold (init , | acc , (_ , v) | f (acc , v)) } }
};
}
