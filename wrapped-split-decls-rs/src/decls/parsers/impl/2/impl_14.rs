use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Parse for TomlSection { fn parse (input : ParseStream) -> Result < Self > { Ok (TomlSection { items : Punctuated :: parse_terminated (input) ? , }) } }
}