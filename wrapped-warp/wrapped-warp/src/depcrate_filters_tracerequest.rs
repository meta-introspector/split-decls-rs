// Generated macro for request (function)
macro_rules! Depcrate_filters_tracerequest {
() => {
// Module: crate::filters::trace
// Provides: {"request"}
// Dependencies: {}
# [doc = " Create a wrapping filter that instruments every request with a `tracing`"] # [doc = " [`Span`] at the [`INFO`] level, containing a summary of the request."] # [doc = " Additionally, if the [`DEBUG`] level is enabled, the span will contain an"] # [doc = " event recording the request's headers."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " let route = warp::any()"] # [doc = "     .map(warp::reply)"] # [doc = "     .with(warp::trace::request());"] # [doc = " ```"] # [doc = ""] # [doc = " [`Span`]: https://docs.rs/tracing/latest/tracing/#spans"] # [doc = " [`INFO`]: https://docs.rs/tracing/0.1.16/tracing/struct.Level.html#associatedconstant.INFO"] # [doc = " [`DEBUG`]: https://docs.rs/tracing/0.1.16/tracing/struct.Level.html#associatedconstant.DEBUG"] pub fn request () -> Trace < impl Fn (Info < '_ >) -> Span + Clone > { use tracing :: field :: { display , Empty } ; trace (| info : Info < '_ > | { let span = tracing :: info_span ! ("request" , remote . addr = Empty , method = % info . method () , path = % info . path () , version = ? info . route . version () , referer = Empty ,) ; if let Some (referer) = info . referer () { span . record ("referer" , & display (referer)) ; } tracing :: debug ! (parent : & span , "received request") ; span }) }
};
}
