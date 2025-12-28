use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Implements `tracing_core::field::Visit` for some `serde::ser::SerializeMap`."] # [derive (Debug)] pub struct SerdeMapVisitor < S : SerializeMap > { serializer : S , state : Result < () , S :: Error > , }
}