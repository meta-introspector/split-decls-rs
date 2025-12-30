// Generated macro for impl_222 (impl)
macro_rules! Depcrate_tree_indeximpl_222 {
() => {
// Module: crate::tree_index
// Provides: {"impl_222"}
// Dependencies: {}
impl < K , V , Q : ? Sized , R : RangeBounds < Q > > Debug for Range < '_ , '_ , K , V , Q , R > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Range") . field ("root" , & self . root) . field ("leaf_scanner" , & self . leaf_scanner) . field ("check_lower_bound" , & self . check_lower_bound) . field ("check_upper_bound" , & self . check_upper_bound) . finish () } }
};
}
