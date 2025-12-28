use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Ord for ExternAbi { fn cmp (& self , rhs : & Self) -> Ordering { self . as_str () . cmp (rhs . as_str ()) } }