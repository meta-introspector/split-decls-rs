// Generated macro for trim_trailing_slash (function)
macro_rules! Depcrate_normalize_pathtrim_trailing_slash {
() => {
// Module: crate::normalize_path
// Provides: {"trim_trailing_slash"}
// Dependencies: {}
fn trim_trailing_slash (uri : & mut Uri) { if ! uri . path () . ends_with ('/') && ! uri . path () . starts_with ("//") { return ; } let new_path = format ! ("/{}" , uri . path () . trim_matches ('/')) ; let mut parts = uri . clone () . into_parts () ; let new_path_and_query = if let Some (path_and_query) = & parts . path_and_query { let new_path_and_query = if let Some (query) = path_and_query . query () { Cow :: Owned (format ! ("{}?{}" , new_path , query)) } else { new_path . into () } . parse () . unwrap () ; Some (new_path_and_query) } else { None } ; parts . path_and_query = new_path_and_query ; if let Ok (new_uri) = Uri :: from_parts (parts) { * uri = new_uri ; } }
};
}
