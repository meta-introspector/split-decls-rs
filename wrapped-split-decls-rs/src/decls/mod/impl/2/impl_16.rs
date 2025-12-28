use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'g , N : Debug , E : Debug > AdjacentEdges < 'g , N , E > { fn targets (self) -> impl Iterator < Item = NodeIndex > { self . map (| (_ , edge) | edge . target) } fn sources (self) -> impl Iterator < Item = NodeIndex > { self . map (| (_ , edge) | edge . source) } }
}