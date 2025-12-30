// Generated macro for impl_616 (impl)
macro_rules! Depcrate_ty_infoimpl_616 {
() => {
// Module: crate::ty_info
// Provides: {"impl_616"}
// Dependencies: {}
impl < T : Ord > PartialOrd for WithCachedTypeInfo < T > { fn partial_cmp (& self , other : & WithCachedTypeInfo < T >) -> Option < Ordering > { Some (self . internee . cmp (& other . internee)) } }
};
}
