use serde::{Deserialize, Serialize};
use std::collections::HashMap;

unsafe impl < T : Send > Send for TypedArena < T > { }