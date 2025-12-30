// Generated macro for get_skip_names (function)
macro_rules! Depcrate_skipget_skip_names {
() => {
// Module: crate::skip
// Provides: {"get_skip_names"}
// Dependencies: {}
fn get_skip_names (kind : & str , attrs : & [ast :: Attribute]) -> Vec < String > { let mut skip_names = vec ! [] ; let path = format ! ("{RUSTFMT}::{SKIP}::{kind}") ; for attr in attrs { if let ast :: AttrKind :: Normal (normal) = & attr . kind { if pprust :: path_to_string (& normal . item . path) != path { continue ; } } if let Some (list) = attr . meta_item_list () { for meta_item_inner in list { if let Some (name) = meta_item_inner . ident () { skip_names . push (name . to_string ()) ; } } } } skip_names }
};
}
