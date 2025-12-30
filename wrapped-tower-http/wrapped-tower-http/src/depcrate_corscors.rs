// Generated macro for Cors (struct)
macro_rules! Depcrate_corsCors {
() => {
// Module: crate::cors
// Provides: {"Cors"}
// Dependencies: {}
# [doc = " Middleware which adds headers for [CORS][mdn]."] # [doc = ""] # [doc = " See the [module docs](crate::cors) for an example."] # [doc = ""] # [doc = " [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS"] # [derive (Debug , Clone)] # [must_use] pub struct Cors < S > { inner : S , layer : CorsLayer , }
};
}
