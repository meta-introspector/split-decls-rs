use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone , Debug)] pub struct TransitiveRelationBuilder < T > { elements : FxIndexSet < T > , edges : FxHashSet < Edge > , }
}