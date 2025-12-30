// Generated macro for append_trailing_slash (function)
macro_rules! Depcrate_normalize_pathappend_trailing_slash {
() => {
// Module: crate::normalize_path
// Provides: {"append_trailing_slash"}
// Dependencies: {}
fn append_trailing_slash (uri : & mut Uri) { if uri . path () . ends_with ("/") && ! uri . path () . ends_with ("//") { return ; } let trimmed = uri . path () . trim_matches ('/') ; let new_path = if trimmed . is_empty () { "/" . to_string () } else { format ! ("/{trimmed}/") } ; let mut parts = uri . clone () . into_parts () ; let new_path_and_query = if let Some (path_and_query) = & parts . path_and_query { let new_path_and_query = if let Some (query) = path_and_query . query () { Cow :: Owned (format ! ("{new_path}?{query}")) } else { new_path . into () } . parse () . unwrap () ; Some (new_path_and_query) } else { Some (new_path . parse () . unwrap ()) } ; parts . path_and_query = new_path_and_query ; if let Ok (new_uri) = Uri :: from_parts (parts) { * uri = new_uri ; } }
};
}
