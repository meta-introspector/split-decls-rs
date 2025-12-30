// Generated macro for WithStatus (struct)
macro_rules! Depcrate_replyWithStatus {
() => {
// Module: crate::reply
// Provides: {"WithStatus"}
// Dependencies: {}
# [doc = " Wrap an `impl Reply` to change its `StatusCode`."] # [doc = ""] # [doc = " Returned by `warp::reply::with_status`."] # [derive (Debug)] pub struct WithStatus < T > { reply : T , status : StatusCode , }
};
}
