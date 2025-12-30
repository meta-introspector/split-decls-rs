// Generated macro for method_is (function)
macro_rules! Depcrate_filters_methodmethod_is {
() => {
// Module: crate::filters::method
// Provides: {"method_is"}
// Dependencies: {}
fn method_is < F > (func : F) -> impl Filter < Extract = () , Error = Rejection > + Copy where F : Fn () -> & 'static Method + Copy , { filter_fn (move | route | { let method = func () ; tracing :: trace ! ("method::{:?}?: {:?}" , method , route . method ()) ; if route . method () == method { future :: ok (()) } else { future :: err (crate :: reject :: method_not_allowed ()) } }) }
};
}
