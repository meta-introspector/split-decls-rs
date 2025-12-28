use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Extern { pub fn from_abi (abi : Option < StrLit > , span : Span) -> Extern { match abi { Some (name) => Extern :: Explicit (name , span) , None => Extern :: Implicit (span) , } } pub fn span (self) -> Option < Span > { match self { Extern :: None => None , Extern :: Implicit (span) | Extern :: Explicit (_ , span) => Some (span) , } } }
}