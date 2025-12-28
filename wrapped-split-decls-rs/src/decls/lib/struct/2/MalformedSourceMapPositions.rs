use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone , PartialEq , Eq , Debug)] pub struct MalformedSourceMapPositions { pub name : FileName , pub source_len : usize , pub begin_pos : BytePos , pub end_pos : BytePos , }
}