// Generated macro for filter_fn_one (function)
macro_rules! Depcrate_filterfilter_fn_one {
() => {
// Module: crate::filter
// Provides: {"filter_fn_one"}
// Dependencies: {}
pub (crate) fn filter_fn_one < F , U > (func : F ,) -> impl Filter < Extract = (U :: Ok ,) , Error = U :: Error > + Copy where F : Fn (& mut Route) -> U + Copy , U : TryFuture + Send + 'static , U :: Ok : Send , U :: Error : IsReject , { filter_fn (move | route | func (route) . map_ok (| item | (item ,))) }
};
}
