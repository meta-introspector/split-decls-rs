use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct ExpandedParam (pub ast :: Param) ;
}