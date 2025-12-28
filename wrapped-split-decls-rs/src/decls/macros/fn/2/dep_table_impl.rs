use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn dep_table_impl (input : TokenStream) -> TokenStream { let DepTableInput { name , table_content , .. } = parse_macro_input ! (input as DepTableInput) ; let name_ident = Ident :: new (& name . value () , name . span ()) ; quote ! { # name_ident = { # table_content } } . into () }
}