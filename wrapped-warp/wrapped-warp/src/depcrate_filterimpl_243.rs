// Generated macro for impl_243 (impl)
macro_rules! Depcrate_filterimpl_243 {
() => {
// Module: crate::filter
// Provides: {"impl_243"}
// Dependencies: {}
impl < F , U > FilterBase for FilterFn < F > where F : Fn (& mut Route) -> U , U : TryFuture + Send + 'static , U :: Ok : Tuple + Send , U :: Error : IsReject , { type Extract = U :: Ok ; type Error = U :: Error ; type Future = future :: IntoFuture < U > ; # [inline] fn filter (& self , _ : Internal) -> Self :: Future { route :: with (| route | (self . func) (route)) . into_future () } }
};
}
