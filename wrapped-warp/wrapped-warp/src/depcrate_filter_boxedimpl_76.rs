// Generated macro for impl_76 (impl)
macro_rules! Depcrate_filter_boxedimpl_76 {
() => {
// Module: crate::filter::boxed
// Provides: {"impl_76"}
// Dependencies: {}
impl < T : Tuple + Send > FilterBase for BoxedFilter < T > { type Extract = T ; type Error = Rejection ; type Future = Pin < Box < dyn Future < Output = Result < T , Rejection > > + Send > > ; fn filter (& self , _ : Internal) -> Self :: Future { self . filter . filter (Internal) } }
};
}
