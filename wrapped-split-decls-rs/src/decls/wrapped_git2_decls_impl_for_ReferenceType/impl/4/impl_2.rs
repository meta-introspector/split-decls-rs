use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl ReferenceType { # [doc = " Convert an object type to its string representation."] pub fn str (& self) -> & 'static str { match self { ReferenceType :: Direct => "direct" , ReferenceType :: Symbolic => "symbolic" , } } # [doc = " Convert a raw git_reference_t to a ReferenceType."] pub fn from_raw (raw : raw :: git_reference_t) -> Option < ReferenceType > { match raw { raw :: GIT_REFERENCE_DIRECT => Some (ReferenceType :: Direct) , raw :: GIT_REFERENCE_SYMBOLIC => Some (ReferenceType :: Symbolic) , _ => None , } } }
}