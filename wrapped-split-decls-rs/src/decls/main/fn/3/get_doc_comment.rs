use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn get_doc_comment (attrs : & [Attribute]) -> Option < String > { let mut doc_comments = Vec :: new () ; for attr in attrs { if attr . path () . is_ident ("doc") { if let Meta :: NameValue (nv) = & attr . meta { if let syn :: Expr :: Lit (expr_lit) = & nv . value { if let syn :: Lit :: Str (lit_str) = & expr_lit . lit { doc_comments . push (lit_str . value () . trim () . to_string ()) ; } } } } } if doc_comments . is_empty () { None } else { Some (doc_comments . join ("\n")) } }
}