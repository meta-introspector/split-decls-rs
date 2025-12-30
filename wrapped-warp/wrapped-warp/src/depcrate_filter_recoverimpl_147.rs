// Generated macro for impl_147 (impl)
macro_rules! Depcrate_filter_recoverimpl_147 {
() => {
// Module: crate::filter::recover
// Provides: {"impl_147"}
// Dependencies: {}
impl < T , F > FilterBase for Recover < T , F > where T : Filter , F : Func < T :: Error > + Clone + Send , F :: Output : TryFuture + Send , < F :: Output as TryFuture > :: Error : IsReject , { type Extract = (Either < T :: Extract , (< F :: Output as TryFuture > :: Ok ,) > ,) ; type Error = < F :: Output as TryFuture > :: Error ; type Future = RecoverFuture < T , F > ; # [inline] fn filter (& self , _ : Internal) -> Self :: Future { let idx = route :: with (| route | route . matched_path_index ()) ; RecoverFuture { state : State :: First (self . filter . filter (Internal) , self . callback . clone ()) , original_path_index : PathIndex (idx) , } } }
};
}
