use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct ExpandedPatField (pub ast :: PatField) ;
}