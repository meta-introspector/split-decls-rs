use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct ExpandedAstNodeWrapper < T , Tag > (pub ast :: AstNodeWrapper < T , Tag >) ;
}