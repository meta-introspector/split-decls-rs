use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct ExpandedWherePredicate (pub ast :: WherePredicate) ;
}