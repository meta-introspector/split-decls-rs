// Generated macro for impl_130 (impl)
macro_rules! Depcrate_filter_or_elseimpl_130 {
() => {
// Module: crate::filter::or_else
// Provides: {"impl_130"}
// Dependencies: {}
impl < T , F > FilterBase for OrElse < T , F > where T : Filter , F : Func < T :: Error > + Clone + Send , F :: Output : TryFuture < Ok = T :: Extract > + Send , < F :: Output as TryFuture > :: Error : IsReject , { type Extract = < F :: Output as TryFuture > :: Ok ; type Error = < F :: Output as TryFuture > :: Error ; type Future = OrElseFuture < T , F > ; # [inline] fn filter (& self , _ : Internal) -> Self :: Future { let idx = route :: with (| route | route . matched_path_index ()) ; OrElseFuture { state : State :: First (self . filter . filter (Internal) , self . callback . clone ()) , original_path_index : PathIndex (idx) , } } }
};
}
