use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl HasTokens for Stmt { fn tokens (& self) -> Option < & LazyAttrTokenStream > { self . kind . tokens () } fn tokens_mut (& mut self) -> Option < & mut Option < LazyAttrTokenStream > > { self . kind . tokens_mut () } }
}