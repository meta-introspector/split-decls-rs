use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct AllocatorMethod { pub name : Symbol , pub inputs : & 'static [AllocatorMethodInput] , pub output : AllocatorTy , }
}