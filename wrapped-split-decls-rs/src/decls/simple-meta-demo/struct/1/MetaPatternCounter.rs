use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct MetaPatternCounter { pub counts : std :: collections :: HashMap < String , usize > , }
}