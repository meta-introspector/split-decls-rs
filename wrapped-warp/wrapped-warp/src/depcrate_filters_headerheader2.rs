// Generated macro for header2 (function)
macro_rules! Depcrate_filters_headerheader2 {
() => {
// Module: crate::filters::header
// Provides: {"header2"}
// Dependencies: {}
pub (crate) fn header2 < T : Header + Send + 'static > () -> impl Filter < Extract = One < T > , Error = Rejection > + Copy { filter_fn_one (move | route | { tracing :: trace ! ("header2({:?})" , T :: name ()) ; let route = route . headers () . typed_get () . ok_or_else (| | reject :: invalid_header (T :: name () . as_str ())) ; future :: ready (route) }) }
};
}
