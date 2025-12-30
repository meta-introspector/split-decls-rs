// Generated macro for impl_617 (impl)
macro_rules! Depcrate_ty_infoimpl_617 {
() => {
// Module: crate::ty_info
// Provides: {"impl_617"}
// Dependencies: {}
impl < T : Ord > Ord for WithCachedTypeInfo < T > { fn cmp (& self , other : & WithCachedTypeInfo < T >) -> Ordering { self . internee . cmp (& other . internee) } }
};
}
