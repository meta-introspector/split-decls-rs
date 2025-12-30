// Generated macro for impl_513 (impl)
macro_rules! Depcrate_collections_hash_setimpl_513 {
() => {
// Module: crate::collections::hash::set
// Provides: {"impl_513"}
// Dependencies: {}
# [unstable (feature = "hash_set_entry" , issue = "60896")] impl < T : fmt :: Debug , S > fmt :: Debug for VacantEntry < '_ , T , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("VacantEntry") . field (self . get ()) . finish () } }
};
}
