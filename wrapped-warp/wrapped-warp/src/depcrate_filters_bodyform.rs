// Generated macro for form (function)
macro_rules! Depcrate_filters_bodyform {
() => {
// Module: crate::filters::body
// Provides: {"form"}
// Dependencies: {}
# [doc = " Returns a `Filter` that matches any request and extracts a"] # [doc = " `Future` of a form encoded body."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " This filter is for the simpler `application/x-www-form-urlencoded` format,"] # [doc = " not `multipart/form-data`."] # [doc = ""] # [doc = " # Warning"] # [doc = ""] # [doc = " This does not have a default size limit, it would be wise to use one to"] # [doc = " prevent a overly large request from using too much memory."] # [doc = ""] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashMap;"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " let route = warp::body::content_length_limit(1024 * 32)"] # [doc = "     .and(warp::body::form())"] # [doc = "     .map(|simple_map: HashMap<String, String>| {"] # [doc = "         \"Got a urlencoded body!\""] # [doc = "     });"] # [doc = " ```"] pub fn form < T : DeserializeOwned + Send > () -> impl Filter < Extract = (T ,) , Error = Rejection > + Copy { is_content_type :: < Form > () . and (aggregate ()) . and_then (| buf | async move { Form :: decode (buf) . map_err (| err | { tracing :: debug ! ("request form body error: {}" , err) ; reject :: known (BodyDeserializeError { cause : err }) }) }) }
};
}
