// Generated macro for impl_179 (impl)
macro_rules! Depcrate_filter_thenimpl_179 {
() => {
// Module: crate::filter::then
// Provides: {"impl_179"}
// Dependencies: {}
impl < T , F > FilterBase for Then < T , F > where T : Filter , F : Func < T :: Extract > + Clone + Send , F :: Output : Future + Send , { type Extract = (< F :: Output as Future > :: Output ,) ; type Error = T :: Error ; type Future = ThenFuture < T , F > ; # [inline] fn filter (& self , _ : Internal) -> Self :: Future { ThenFuture { state : State :: First (self . filter . filter (Internal) , self . callback . clone ()) , } } }
};
}
