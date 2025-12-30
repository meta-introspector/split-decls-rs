// Generated macro for filter_segment (function)
macro_rules! Depcrate_filters_pathfilter_segment {
() => {
// Module: crate::filters::path
// Provides: {"filter_segment"}
// Dependencies: {}
fn filter_segment < F , U > (func : F) -> impl Filter < Extract = U , Error = Rejection > + Copy where F : Fn (& str) -> Result < U , Rejection > + Copy , U : Tuple + Send + 'static , { filter_fn (move | route | future :: ready (with_segment (route , func))) }
};
}
