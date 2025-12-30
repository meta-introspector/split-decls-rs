// Generated macro for reply (function)
macro_rules! Depcrate_replyreply {
() => {
// Module: crate::reply
// Provides: {"reply"}
// Dependencies: {}
# [doc = " Returns an empty `Reply` with status code `200 OK`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " // GET /just-ok returns an empty `200 OK`."] # [doc = " let route = warp::path(\"just-ok\")"] # [doc = "     .map(|| {"] # [doc = "         println!(\"got a /just-ok request!\");"] # [doc = "         warp::reply()"] # [doc = "     });"] # [doc = " ```"] # [inline] pub fn reply () -> impl Reply { StatusCode :: OK }
};
}
