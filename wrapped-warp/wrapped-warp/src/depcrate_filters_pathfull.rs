// Generated macro for full (function)
macro_rules! Depcrate_filters_pathfull {
() => {
// Module: crate::filters::path
// Provides: {"full"}
// Dependencies: {}
# [doc = " Returns the full request path, irrespective of other filters."] # [doc = ""] # [doc = " This will return a `FullPath`, which can be stringified to return the"] # [doc = " full path of the request."] # [doc = ""] # [doc = " This is more useful in generic pre/post-processing filters, and should"] # [doc = " probably not be used for request matching/routing."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::{Filter, path::FullPath};"] # [doc = " use std::{collections::HashMap, sync::{Arc, Mutex}};"] # [doc = ""] # [doc = " let counts = Arc::new(Mutex::new(HashMap::new()));"] # [doc = " let access_counter = warp::path::full()"] # [doc = "     .map(move |path: FullPath| {"] # [doc = "         let mut counts = counts.lock().unwrap();"] # [doc = ""] # [doc = "         *counts.entry(path.as_str().to_string())"] # [doc = "             .and_modify(|c| *c += 1)"] # [doc = "             .or_insert(0)"] # [doc = "     });"] # [doc = ""] # [doc = " let route = warp::path(\"foo\")"] # [doc = "     .and(warp::path(\"bar\"))"] # [doc = "     .and(access_counter)"] # [doc = "     .map(|count| {"] # [doc = "         format!(\"This is the {}th visit to this URL!\", count)"] # [doc = "     });"] # [doc = " ```"] pub fn full () -> impl Filter < Extract = One < FullPath > , Error = Infallible > + Copy { filter_fn (move | route | future :: ok (one (FullPath (path_and_query (route))))) }
};
}
