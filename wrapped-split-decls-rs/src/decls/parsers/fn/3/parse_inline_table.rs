use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn parse_inline_table (input : proc_macro2 :: TokenStream) -> Result < HashMap < String , String > > { let inline_table : InlineTable = syn :: parse2 (input) ? ; let mut map = HashMap :: new () ; for item in inline_table . items { if let KeyValue :: Simple (key , val) = item { map . insert (key . to_string () , val . value ()) ; } else { return Err (syn :: Error :: new_spanned (item . get_ident_for_err () , "expected simple key-value pairs in inline table" ,)) ; } } Ok (map) }
}