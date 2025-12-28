use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn has_attr (attrs : & [syn :: Attribute] , name : & str) -> bool { attrs . iter () . any (| a | { if let Ok (i) = a . parse_args :: < Ident > () { if i == name { return true ; } } false }) }
}