// Generated macro for append_slash_on_path (function)
macro_rules! Depcrate_services_fs_serve_dir_open_fileappend_slash_on_path {
() => {
// Module: crate::services::fs::serve_dir::open_file
// Provides: {"append_slash_on_path"}
// Dependencies: {}
fn append_slash_on_path (uri : Uri) -> Result < Uri , OpenFileOutput > { let http :: uri :: Parts { scheme , authority , path_and_query , .. } = uri . into_parts () ; let mut uri_builder = Uri :: builder () ; if let Some (scheme) = scheme { uri_builder = uri_builder . scheme (scheme) ; } if let Some (authority) = authority { uri_builder = uri_builder . authority (authority) ; } let uri_builder = if let Some (path_and_query) = path_and_query { if let Some (query) = path_and_query . query () { uri_builder . path_and_query (format ! ("{}/?{}" , path_and_query . path () , query)) } else { uri_builder . path_and_query (format ! ("{}/" , path_and_query . path ())) } } else { uri_builder . path_and_query ("/") } ; uri_builder . build () . map_err (| err | { tracing :: error ! (? err , "redirect uri failed to build") ; OpenFileOutput :: InvalidRedirectUri }) }
};
}
