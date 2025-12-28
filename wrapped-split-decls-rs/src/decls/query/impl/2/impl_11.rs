use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : Parse > Parse for List < T > { fn parse (input : ParseStream < '_ >) -> Result < Self > { let mut list = Vec :: new () ; while ! input . is_empty () { list . push (input . parse () ?) ; } Ok (List (list)) } }
}