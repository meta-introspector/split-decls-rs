use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone , Copy)] struct InlineParent { lo : u32 , len_with_tag : u16 , parent : u16 , }
}