use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
unsafe impl < T : DynSync + DynSend > DynSync for FreezeLock < T > { }
}