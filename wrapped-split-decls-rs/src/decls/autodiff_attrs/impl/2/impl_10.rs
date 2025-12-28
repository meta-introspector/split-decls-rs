use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl DiffMode { pub fn is_rev (& self) -> bool { matches ! (self , DiffMode :: Reverse) } pub fn is_fwd (& self) -> bool { matches ! (self , DiffMode :: Forward) } }