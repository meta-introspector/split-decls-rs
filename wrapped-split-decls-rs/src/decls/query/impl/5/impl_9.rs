use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Parse for Query { fn parse (input : ParseStream < '_ >) -> Result < Self > { let mut doc_comments = check_attributes (input . call (Attribute :: parse_outer) ?) ? ; input . parse :: < kw :: query > () ? ; let name : Ident = input . parse () ? ; let arg_content ; parenthesized ! (arg_content in input) ; let key = Pat :: parse_single (& arg_content) ? ; arg_content . parse :: < Token ! [:] > () ? ; let arg = arg_content . parse () ? ; let _ = arg_content . parse :: < Option < Token ! [,] > > () ? ; let result = input . parse () ? ; let content ; braced ! (content in input) ; let modifiers = parse_query_modifiers (& content) ? ; if doc_comments . is_empty () { doc_comments . push (doc_comment_from_desc (& modifiers . desc . 1) ?) ; } Ok (Query { doc_comments , modifiers , name , key , arg , result }) } }
}