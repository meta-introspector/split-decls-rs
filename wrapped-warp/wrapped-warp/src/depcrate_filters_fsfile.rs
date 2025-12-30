// Generated macro for file (function)
macro_rules! Depcrate_filters_fsfile {
() => {
// Module: crate::filters::fs
// Provides: {"file"}
// Dependencies: {}
# [doc = " Creates a `Filter` that serves a File at the `path`."] # [doc = ""] # [doc = " Does not filter out based on any information of the request. Always serves"] # [doc = " the file at the exact `path` provided. Thus, this can be used to serve a"] # [doc = " single file with `GET`s, but could also be used in combination with other"] # [doc = " filters, such as after validating in `POST` request, wanting to return a"] # [doc = " specific file as the body."] # [doc = ""] # [doc = " For serving a directory, see [dir]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " // Always serves this file from the file system."] # [doc = " let route = warp::fs::file(\"/www/static/app.js\");"] # [doc = " ```"] pub fn file (path : impl Into < PathBuf >) -> impl FilterClone < Extract = One < File > , Error = Rejection > { let path = Arc :: new (path . into ()) ; crate :: any () . map (move | | { tracing :: trace ! ("file: {:?}" , path) ; ArcPath (path . clone ()) }) . and (conditionals ()) . and_then (file_reply) }
};
}
