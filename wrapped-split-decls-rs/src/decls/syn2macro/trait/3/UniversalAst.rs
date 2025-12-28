use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [doc = " Universal AST execution context trait"] pub trait UniversalAst { type TokenStream ; type Item ; type Error ; fn parse_item (& self , input : Self :: TokenStream) -> Result < Self :: Item , Self :: Error > ; fn transform_item (& self , item : Self :: Item) -> Result < Self :: Item , Self :: Error > ; fn generate_code (& self , item : Self :: Item) -> Self :: TokenStream ; }
}