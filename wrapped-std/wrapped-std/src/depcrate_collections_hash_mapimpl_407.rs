// Generated macro for impl_407 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_407 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_407"}
// Dependencies: {}
# [stable (feature = "map_values_mut" , since = "1.10.0")] impl < 'a , K , V > Iterator for ValuesMut < 'a , K , V > { type Item = & 'a mut V ; # [inline] fn next (& mut self) -> Option < & 'a mut V > { self . inner . next () . map (| (_ , v) | v) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } # [inline] fn count (self) -> usize { self . inner . len () } # [inline] fn fold < B , F > (self , init : B , mut f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . inner . fold (init , | acc , (_ , v) | f (acc , v)) } }
};
}
