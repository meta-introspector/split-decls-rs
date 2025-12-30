// Generated macro for dir (function)
macro_rules! Depcrate_filters_fsdir {
() => {
// Module: crate::filters::fs
// Provides: {"dir"}
// Dependencies: {}
# [doc = " Creates a `Filter` that serves a directory at the base `path` joined"] # [doc = " by the request path."] # [doc = ""] # [doc = " This can be used to serve \"static files\" from a directory. By far the most"] # [doc = " common pattern of serving static files is for `GET` requests, so this"] # [doc = " filter automatically includes a `GET` check."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " // Matches requests that start with `/static`,"] # [doc = " // and then uses the rest of that path to lookup"] # [doc = " // and serve a file from `/www/static`."] # [doc = " let route = warp::path(\"static\")"] # [doc = "     .and(warp::fs::dir(\"/www/static\"));"] # [doc = ""] # [doc = " // For example:"] # [doc = " // - `GET /static/app.js` would serve the file `/www/static/app.js`"] # [doc = " // - `GET /static/css/app.css` would serve the file `/www/static/css/app.css`"] # [doc = " ```"] pub fn dir (path : impl Into < PathBuf >) -> impl FilterClone < Extract = One < File > , Error = Rejection > { let base = Arc :: new (path . into ()) ; crate :: get () . or (crate :: head ()) . unify () . and (path_from_tail (base)) . and (conditionals ()) . and_then (file_reply) }
};
}
