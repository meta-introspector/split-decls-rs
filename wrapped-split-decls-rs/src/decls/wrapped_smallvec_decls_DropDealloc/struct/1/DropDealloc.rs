use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct DropDealloc { ptr : NonNull < u8 > , size_bytes : usize , align : usize , }
}