use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Parse for DepTableInput { fn parse (input : ParseStream) -> Result < Self > { let name : LitStr = input . parse () ? ; let comma_token : Token ! [,] = input . parse () ? ; let table_content : proc_macro2 :: TokenStream = input . parse () ? ; Ok (DepTableInput { name , _comma_token : comma_token , table_content , }) } }
}