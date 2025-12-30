// Generated macro for optional (function)
macro_rules! Depcrate_filters_extoptional {
() => {
// Module: crate::filters::ext
// Provides: {"optional"}
// Dependencies: {}
# [doc = " Get a previously set extension of the current route."] # [doc = ""] # [doc = " If the extension doesn't exist, it yields `None`."] pub fn optional < T : Clone + Send + Sync + 'static > () -> impl Filter < Extract = (Option < T > ,) , Error = Infallible > + Copy { filter_fn_one (| route | future :: ok (route . extensions () . get :: < T > () . cloned ())) }
};
}
