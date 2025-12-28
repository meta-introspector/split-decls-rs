use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Info associated with a text range."] # [derive (Debug , UpmapFromRaFixture)] pub struct RangeInfo < T > { pub range : TextRange , pub info : T , }
}