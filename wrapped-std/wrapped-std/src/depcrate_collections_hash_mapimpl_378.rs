// Generated macro for impl_378 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_378 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_378"}
// Dependencies: {}
# [stable (feature = "debug_hash_map" , since = "1.12.0")] impl < K : Debug , V : Debug > Debug for Entry < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Vacant (ref v) => f . debug_tuple ("Entry") . field (v) . finish () , Occupied (ref o) => f . debug_tuple ("Entry") . field (o) . finish () , } } }
};
}
