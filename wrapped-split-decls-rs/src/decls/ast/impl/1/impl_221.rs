use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl VisibilityKind { pub fn is_pub (& self) -> bool { matches ! (self , VisibilityKind :: Public) } }