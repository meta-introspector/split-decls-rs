// Generated macro for impl_511 (impl)
macro_rules! Depcrate_collections_hash_setimpl_511 {
() => {
// Module: crate::collections::hash::set
// Provides: {"impl_511"}
// Dependencies: {}
# [unstable (feature = "hash_set_entry" , issue = "60896")] impl < T : fmt :: Debug , S > fmt :: Debug for OccupiedEntry < '_ , T , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("OccupiedEntry") . field ("value" , self . get ()) . finish () } }
};
}
