// Generated macro for impl_192 (impl)
macro_rules! Depcrate_filter_unifyimpl_192 {
() => {
// Module: crate::filter::unify
// Provides: {"impl_192"}
// Dependencies: {}
impl < F , T > FilterBase for Unify < F > where F : Filter < Extract = (Either < T , T > ,) > , T : Tuple , { type Extract = T ; type Error = F :: Error ; type Future = UnifyFuture < F :: Future > ; # [inline] fn filter (& self , _ : Internal) -> Self :: Future { UnifyFuture { inner : self . filter . filter (Internal) , } } }
};
}
