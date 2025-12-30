// Generated macro for CorsLayer (struct)
macro_rules! Depcrate_corsCorsLayer {
() => {
// Module: crate::cors
// Provides: {"CorsLayer"}
// Dependencies: {}
# [doc = " Layer that applies the [`Cors`] middleware which adds headers for [CORS][mdn]."] # [doc = ""] # [doc = " See the [module docs](crate::cors) for an example."] # [doc = ""] # [doc = " [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS"] # [derive (Debug , Clone)] # [must_use] pub struct CorsLayer { allow_credentials : AllowCredentials , allow_headers : AllowHeaders , allow_methods : AllowMethods , allow_origin : AllowOrigin , allow_private_network : AllowPrivateNetwork , expose_headers : ExposeHeaders , max_age : MaxAge , vary : Vary , }
};
}
