// Generated macro for impl_216 (impl)
macro_rules! Depcrate_tree_indeximpl_216 {
() => {
// Module: crate::tree_index
// Provides: {"impl_216"}
// Dependencies: {}
impl < K , V > Debug for Iter < '_ , '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Iter") . field ("root" , & self . root) . field ("leaf_scanner" , & self . leaf_scanner) . finish () } }
};
}
