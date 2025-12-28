use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn parse_string_list (input : proc_macro2 :: TokenStream) -> Result < Vec < String > > { let string_list : BracketedStringList = syn :: parse2 (input) ? ; Ok (string_list . list . into_iter () . map (| s | s . value ()) . collect ()) }
}