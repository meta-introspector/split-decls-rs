use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct AllocatorMethodInput { pub name : & 'static str , pub ty : AllocatorTy , }
}