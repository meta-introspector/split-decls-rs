// Generated macro for impl_388 (impl)
macro_rules! Depcrate_filters_fsimpl_388 {
() => {
// Module: crate::filters::fs
// Provides: {"impl_388"}
// Dependencies: {}
impl File { # [doc = " Extract the `&Path` of the file this `Response` delivers."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " The example below changes the Content-Type response header for every file called `video.mp4`."] # [doc = ""] # [doc = " ```"] # [doc = " use warp::{Filter, reply::Reply};"] # [doc = ""] # [doc = " let route = warp::path(\"static\")"] # [doc = "     .and(warp::fs::dir(\"/www/static\"))"] # [doc = "     .map(|reply: warp::filters::fs::File| {"] # [doc = "         if reply.path().ends_with(\"video.mp4\") {"] # [doc = "             warp::reply::with_header(reply, \"Content-Type\", \"video/mp4\").into_response()"] # [doc = "         } else {"] # [doc = "             reply.into_response()"] # [doc = "         }"] # [doc = "     });"] # [doc = " ```"] pub fn path (& self) -> & Path { self . path . as_ref () } }
};
}
