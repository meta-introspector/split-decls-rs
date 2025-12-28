use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Parse for Symbol { fn parse (input : ParseStream < '_ >) -> Result < Self > { let name = input . parse () ? ; let colon_token : Option < Token ! [:] > = input . parse () ? ; let value = if colon_token . is_some () { input . parse () ? } else { Value :: SameAsName } ; Ok (Symbol { name , value }) } }
}