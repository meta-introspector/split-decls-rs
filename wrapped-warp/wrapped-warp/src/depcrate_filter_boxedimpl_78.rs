// Generated macro for impl_78 (impl)
macro_rules! Depcrate_filter_boxedimpl_78 {
() => {
// Module: crate::filter::boxed
// Provides: {"impl_78"}
// Dependencies: {}
impl < F > FilterBase for BoxingFilter < F > where F : Filter , F :: Future : Send + 'static , { type Extract = F :: Extract ; type Error = F :: Error ; type Future = Pin < Box < dyn Future < Output = Result < Self :: Extract , Self :: Error > > + Send > > ; fn filter (& self , _ : Internal) -> Self :: Future { Box :: pin (self . filter . filter (Internal) . into_future ()) } }
};
}
