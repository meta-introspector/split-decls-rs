// Generated macro for cors (function)
macro_rules! Depcrate_filters_corscors {
() => {
// Module: crate::filters::cors
// Provides: {"cors"}
// Dependencies: {}
# [doc = " Create a wrapping [`Filter`] that exposes [CORS][] behavior for a wrapped"] # [doc = " filter."] # [doc = ""] # [doc = " [CORS]: https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " let cors = warp::cors()"] # [doc = "     .allow_origin(\"https://hyper.rs\")"] # [doc = "     .allow_methods(vec![\"GET\", \"POST\", \"DELETE\"]);"] # [doc = ""] # [doc = " let route = warp::any()"] # [doc = "     .map(warp::reply)"] # [doc = "     .with(cors);"] # [doc = " ```"] # [doc = " If you want to allow any route:"] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = " let cors = warp::cors()"] # [doc = "     .allow_any_origin();"] # [doc = " ```"] # [doc = " You can find more usage examples [here](https://github.com/seanmonstar/warp/blob/7fa54eaecd0fe12687137372791ff22fc7995766/tests/cors.rs)."] pub fn cors () -> Builder { Builder { credentials : false , allowed_headers : HashSet :: new () , exposed_headers : HashSet :: new () , max_age : None , methods : HashSet :: new () , origins : None , } }
};
}
