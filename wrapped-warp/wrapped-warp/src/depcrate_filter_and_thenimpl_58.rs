// Generated macro for impl_58 (impl)
macro_rules! Depcrate_filter_and_thenimpl_58 {
() => {
// Module: crate::filter::and_then
// Provides: {"impl_58"}
// Dependencies: {}
impl < T , F > FilterBase for AndThen < T , F > where T : Filter , F : Func < T :: Extract > + Clone + Send , F :: Output : TryFuture + Send , < F :: Output as TryFuture > :: Error : CombineRejection < T :: Error > , { type Extract = (< F :: Output as TryFuture > :: Ok ,) ; type Error = < < F :: Output as TryFuture > :: Error as CombineRejection < T :: Error > > :: One ; type Future = AndThenFuture < T , F > ; # [inline] fn filter (& self , _ : Internal) -> Self :: Future { AndThenFuture { state : State :: First (self . filter . filter (Internal) , self . callback . clone ()) , } } }
};
}
