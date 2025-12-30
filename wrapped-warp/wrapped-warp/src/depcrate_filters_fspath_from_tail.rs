// Generated macro for path_from_tail (function)
macro_rules! Depcrate_filters_fspath_from_tail {
() => {
// Module: crate::filters::fs
// Provides: {"path_from_tail"}
// Dependencies: {}
fn path_from_tail (base : Arc < PathBuf > ,) -> impl FilterClone < Extract = One < ArcPath > , Error = Rejection > { crate :: path :: tail () . and_then (move | tail : crate :: path :: Tail | { future :: ready (sanitize_path (base . as_ref () , tail . as_str ())) . and_then (| mut buf | async { let is_dir = tokio :: fs :: metadata (buf . clone ()) . await . map (| m | m . is_dir ()) . unwrap_or (false) ; if is_dir { tracing :: debug ! ("dir: appending index.html to directory path") ; buf . push ("index.html") ; } tracing :: trace ! ("dir: {:?}" , buf) ; Ok (ArcPath (Arc :: new (buf))) }) }) }
};
}
