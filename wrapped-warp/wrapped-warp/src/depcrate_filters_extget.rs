// Generated macro for get (function)
macro_rules! Depcrate_filters_extget {
() => {
// Module: crate::filters::ext
// Provides: {"get"}
// Dependencies: {}
# [doc = " Get a previously set extension of the current route."] # [doc = ""] # [doc = " If the extension doesn't exist, this rejects with a `MissingExtension`."] pub fn get < T : Clone + Send + Sync + 'static > () -> impl Filter < Extract = (T ,) , Error = Rejection > + Copy { filter_fn_one (| route | { let route = route . extensions () . get :: < T > () . cloned () . ok_or_else (| | reject :: known (MissingExtension { _p : () })) ; future :: ready (route) }) }
};
}
