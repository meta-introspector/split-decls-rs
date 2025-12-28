use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Ord for Utf8Path { fn cmp (& self , other : & Utf8Path) -> Ordering { self . components () . cmp (other . components ()) } }