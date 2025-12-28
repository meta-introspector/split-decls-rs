use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Modifiers on a trait bound like `[const]`, `?` and `!`."] # [derive (Copy , Clone , PartialEq , Eq , Encodable , Decodable , Debug , Walkable)] pub struct TraitBoundModifiers { pub constness : BoundConstness , pub asyncness : BoundAsyncness , pub polarity : BoundPolarity , }
}