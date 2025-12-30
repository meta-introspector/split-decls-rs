// Generated macro for filter_fn (function)
macro_rules! Depcrate_filterfilter_fn {
() => {
// Module: crate::filter
// Provides: {"filter_fn"}
// Dependencies: {}
pub (crate) fn filter_fn < F , U > (func : F) -> FilterFn < F > where F : Fn (& mut Route) -> U , U : TryFuture , U :: Ok : Tuple , U :: Error : IsReject , { FilterFn { func } }
};
}
