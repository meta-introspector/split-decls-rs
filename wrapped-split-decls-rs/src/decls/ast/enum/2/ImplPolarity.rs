use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Copy , Clone , PartialEq , Encodable , Decodable , HashStable_Generic , Walkable)] pub enum ImplPolarity { # [doc = " `impl Trait for Type`"] Positive , # [doc = " `impl !Trait for Type`"] Negative (Span) , }
}