// Generated macro for impl_415 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_415 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_415"}
// Dependencies: {}
# [stable (feature = "map_into_keys_values" , since = "1.54.0")] impl < K , V > Iterator for IntoValues < K , V > { type Item = V ; # [inline] fn next (& mut self) -> Option < V > { self . inner . next () . map (| (_ , v) | v) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } # [inline] fn count (self) -> usize { self . inner . len () } # [inline] fn fold < B , F > (self , init : B , mut f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . inner . fold (init , | acc , (_ , v) | f (acc , v)) } }
};
}
