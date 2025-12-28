use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < & str > for SmolStr { # [inline] fn from (s : & str) -> SmolStr { SmolStr :: new (s) } }