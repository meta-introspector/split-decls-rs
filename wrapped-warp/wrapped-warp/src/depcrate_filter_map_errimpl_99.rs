// Generated macro for impl_99 (impl)
macro_rules! Depcrate_filter_map_errimpl_99 {
() => {
// Module: crate::filter::map_err
// Provides: {"impl_99"}
// Dependencies: {}
impl < T , F , E > FilterBase for MapErr < T , F > where T : Filter , F : Fn (T :: Error) -> E + Clone + Send , E : IsReject , { type Extract = T :: Extract ; type Error = E ; type Future = MapErrFuture < T , F > ; # [inline] fn filter (& self , _ : Internal) -> Self :: Future { MapErrFuture { extract : self . filter . filter (Internal) , callback : self . callback . clone () , } } }
};
}
