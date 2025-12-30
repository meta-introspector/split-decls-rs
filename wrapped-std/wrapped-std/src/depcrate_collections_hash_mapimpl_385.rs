// Generated macro for impl_385 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_385 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_385"}
// Dependencies: {}
# [unstable (feature = "map_try_insert" , issue = "82766")] impl < 'a , K : Debug , V : Debug > fmt :: Display for OccupiedError < 'a , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "failed to insert {:?}, key {:?} already exists with value {:?}" , self . value , self . entry . key () , self . entry . get () ,) } }
};
}
