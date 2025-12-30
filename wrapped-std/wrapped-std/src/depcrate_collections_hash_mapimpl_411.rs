// Generated macro for impl_411 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_411 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_411"}
// Dependencies: {}
# [stable (feature = "map_into_keys_values" , since = "1.54.0")] impl < K , V > Iterator for IntoKeys < K , V > { type Item = K ; # [inline] fn next (& mut self) -> Option < K > { self . inner . next () . map (| (k , _) | k) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } # [inline] fn count (self) -> usize { self . inner . len () } # [inline] fn fold < B , F > (self , init : B , mut f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . inner . fold (init , | acc , (k , _) | f (acc , k)) } }
};
}
