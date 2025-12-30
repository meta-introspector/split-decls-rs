// Generated macro for impl_87 (impl)
macro_rules! Depcrate_filter_mapimpl_87 {
() => {
// Module: crate::filter::map
// Provides: {"impl_87"}
// Dependencies: {}
impl < T , F > FilterBase for Map < T , F > where T : Filter , F : Func < T :: Extract > + Clone + Send , { type Extract = (F :: Output ,) ; type Error = T :: Error ; type Future = MapFuture < T , F > ; # [inline] fn filter (& self , _ : Internal) -> Self :: Future { MapFuture { extract : self . filter . filter (Internal) , callback : self . callback . clone () , } } }
};
}
