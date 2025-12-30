// Generated macro for impl_509 (impl)
macro_rules! Depcrate_collections_hash_setimpl_509 {
() => {
// Module: crate::collections::hash::set
// Provides: {"impl_509"}
// Dependencies: {}
# [unstable (feature = "hash_set_entry" , issue = "60896")] impl < T : fmt :: Debug , S > fmt :: Debug for Entry < '_ , T , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Entry :: Vacant (ref v) => f . debug_tuple ("Entry") . field (v) . finish () , Entry :: Occupied (ref o) => f . debug_tuple ("Entry") . field (o) . finish () , } } }
};
}
