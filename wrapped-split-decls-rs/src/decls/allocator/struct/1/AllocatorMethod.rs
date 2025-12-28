use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct AllocatorMethod { pub name : Symbol , pub inputs : & 'static [AllocatorMethodInput] , pub output : AllocatorTy , }