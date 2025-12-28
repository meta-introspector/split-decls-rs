use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl PartialOrd for CodeBlock { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }