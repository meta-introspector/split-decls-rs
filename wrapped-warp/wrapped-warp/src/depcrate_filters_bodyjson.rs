// Generated macro for json (function)
macro_rules! Depcrate_filters_bodyjson {
() => {
// Module: crate::filters::body
// Provides: {"json"}
// Dependencies: {}
# [doc = " Returns a `Filter` that matches any request and extracts a `Future` of a"] # [doc = " JSON-decoded body."] # [doc = ""] # [doc = " # Warning"] # [doc = ""] # [doc = " This does not have a default size limit, it would be wise to use one to"] # [doc = " prevent a overly large request from using too much memory."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashMap;"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " let route = warp::body::content_length_limit(1024 * 32)"] # [doc = "     .and(warp::body::json())"] # [doc = "     .map(|simple_map: HashMap<String, String>| {"] # [doc = "         \"Got a JSON body!\""] # [doc = "     });"] # [doc = " ```"] pub fn json < T : DeserializeOwned + Send > () -> impl Filter < Extract = (T ,) , Error = Rejection > + Copy { is_content_type :: < Json > () . and (bytes ()) . and_then (| buf | async move { Json :: decode (buf) . map_err (| err | { tracing :: debug ! ("request json body error: {}" , err) ; reject :: known (BodyDeserializeError { cause : err }) }) }) }
};
}
