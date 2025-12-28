use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone , PartialEq , Eq , Debug)] pub struct DistinctSources { pub begin : (FileName , BytePos) , pub end : (FileName , BytePos) , }
}