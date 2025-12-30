// Generated macro for optional2 (function)
macro_rules! Depcrate_filters_headeroptional2 {
() => {
// Module: crate::filters::header
// Provides: {"optional2"}
// Dependencies: {}
pub (crate) fn optional2 < T > () -> impl Filter < Extract = One < Option < T > > , Error = Infallible > + Copy where T : Header + Send + 'static , { filter_fn_one (move | route | future :: ready (Ok (route . headers () . typed_get ()))) }
};
}
