// Generated macro for impl_203 (impl)
macro_rules! Depcrate_filter_untuple_oneimpl_203 {
() => {
// Module: crate::filter::untuple_one
// Provides: {"impl_203"}
// Dependencies: {}
impl < F , T > FilterBase for UntupleOne < F > where F : Filter < Extract = (T ,) > , T : Tuple , { type Extract = T ; type Error = F :: Error ; type Future = UntupleOneFuture < F > ; # [inline] fn filter (& self , _ : Internal) -> Self :: Future { UntupleOneFuture { extract : self . filter . filter (Internal) , } } }
};
}
