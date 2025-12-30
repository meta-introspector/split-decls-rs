// Generated macro for body (function)
macro_rules! Depcrate_filters_bodybody {
() => {
// Module: crate::filters::body
// Provides: {"body"}
// Dependencies: {}
pub (crate) fn body () -> impl Filter < Extract = (Body ,) , Error = Rejection > + Copy { filter_fn_one (| route | { future :: ready (route . take_body () . ok_or_else (| | { tracing :: error ! ("request body already taken in previous filter") ; reject :: known (BodyConsumedMultipleTimes { _p : () }) })) }) }
};
}
