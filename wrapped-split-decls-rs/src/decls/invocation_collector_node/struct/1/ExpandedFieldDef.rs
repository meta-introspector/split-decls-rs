use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct ExpandedFieldDef (pub ast :: FieldDef) ;
}