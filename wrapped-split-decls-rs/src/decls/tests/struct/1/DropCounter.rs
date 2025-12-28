use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct DropCounter < 'a > { count : & 'a Cell < u32 > , }
}