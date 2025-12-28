use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct ExpandedExprField (pub ast :: ExprField) ;
}