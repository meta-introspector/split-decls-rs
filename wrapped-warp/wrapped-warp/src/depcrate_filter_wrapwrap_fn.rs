// Generated macro for wrap_fn (function)
macro_rules! Depcrate_filter_wrapwrap_fn {
() => {
// Module: crate::filter::wrap
// Provides: {"wrap_fn"}
// Dependencies: {}
# [doc = " Combines received filter with pre and after filters"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use crate::warp::Filter;"] # [doc = ""] # [doc = " let route = warp::any()"] # [doc = "     .map(|| \"hello world\")"] # [doc = "     .with(warp::wrap_fn(|filter| filter));"] # [doc = " ```"] # [doc = ""] # [doc = " You can find the full example in the [usage example](https://github.com/seanmonstar/warp/blob/master/examples/wrapping.rs)."] pub fn wrap_fn < F , T , U > (func : F) -> WrapFn < F > where F : Fn (T) -> U , T : Filter , U : Filter , { WrapFn { func } }
};
}
