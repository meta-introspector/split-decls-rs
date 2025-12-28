use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Deref for AbiAlign { type Target = Align ; fn deref (& self) -> & Self :: Target { & self . abi } }
}