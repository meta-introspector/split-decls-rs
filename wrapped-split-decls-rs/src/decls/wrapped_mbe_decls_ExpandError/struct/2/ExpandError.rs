use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , PartialEq , Eq , Clone , Hash)] pub struct ExpandError { pub inner : Arc < (Span , ExpandErrorKind) > , }
}