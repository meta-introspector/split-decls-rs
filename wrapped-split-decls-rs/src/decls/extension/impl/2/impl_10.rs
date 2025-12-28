use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Parse for ExtensionAttr { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { let vis = input . parse () ? ; let _ : Token ! [trait] = input . parse () ? ; let trait_ = input . parse () ? ; Ok (ExtensionAttr { vis , trait_ }) } }
}