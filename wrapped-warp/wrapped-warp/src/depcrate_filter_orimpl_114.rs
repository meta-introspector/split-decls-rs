// Generated macro for impl_114 (impl)
macro_rules! Depcrate_filter_orimpl_114 {
() => {
// Module: crate::filter::or
// Provides: {"impl_114"}
// Dependencies: {}
impl < T , U > FilterBase for Or < T , U > where T : Filter , U : Filter + Clone + Send , U :: Error : CombineRejection < T :: Error > , { type Extract = (Either < T :: Extract , U :: Extract > ,) ; type Error = Combined < U :: Error , T :: Error > ; type Future = EitherFuture < T , U > ; fn filter (& self , _ : Internal) -> Self :: Future { let idx = route :: with (| route | route . matched_path_index ()) ; EitherFuture { state : State :: First (self . first . filter (Internal) , self . second . clone ()) , original_path_index : PathIndex (idx) , } } }
};
}
