use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Default)] pub struct PlaceholderExpander { expanded_fragments : FxHashMap < ast :: NodeId , AstFragment > , }
}