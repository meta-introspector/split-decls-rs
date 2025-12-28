use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct InternerInner { arena : DroplessArena , byte_strs : FxIndexSet < & 'static [u8] > , }
}