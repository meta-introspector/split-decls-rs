use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < & mut str > for SmolStr { # [inline] fn from (s : & mut str) -> SmolStr { SmolStr :: new (s) } }