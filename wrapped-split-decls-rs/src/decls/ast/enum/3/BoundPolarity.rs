use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " The polarity of a trait bound."] # [derive (Copy , Clone , PartialEq , Eq , Encodable , Decodable , Debug , Hash)] # [derive (HashStable_Generic , Walkable)] pub enum BoundPolarity { # [doc = " `Type: Trait`"] Positive , # [doc = " `Type: !Trait`"] Negative (Span) , # [doc = " `Type: ?Trait`"] Maybe (Span) , }
}