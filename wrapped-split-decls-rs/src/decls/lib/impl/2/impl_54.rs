use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl FnContext { # [doc = " Returns `true` if the closure was called from a different thread"] # [doc = " than it was provided from."] # [inline] pub fn migrated (& self) -> bool { self . migrated } }