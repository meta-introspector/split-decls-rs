use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone , Debug , PartialEq , Eq)] enum SmolStrBuilderRepr { Inline { len : usize , buf : [u8 ; INLINE_CAP] } , Heap (String) , }
}