// Generated macro for with_status (function)
macro_rules! Depcrate_replywith_status {
() => {
// Module: crate::reply
// Provides: {"with_status"}
// Dependencies: {}
# [doc = " Wrap an `impl Reply` to change its `StatusCode`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " let route = warp::any()"] # [doc = "     .map(warp::reply)"] # [doc = "     .map(|reply| {"] # [doc = "         warp::reply::with_status(reply, warp::http::StatusCode::CREATED)"] # [doc = "     });"] # [doc = " ```"] pub fn with_status < T : Reply > (reply : T , status : StatusCode) -> WithStatus < T > { WithStatus { reply , status } }
};
}
