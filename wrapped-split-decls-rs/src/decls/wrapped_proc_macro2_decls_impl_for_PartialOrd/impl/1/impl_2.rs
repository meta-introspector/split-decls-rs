use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl PartialOrd for Ident { fn partial_cmp (& self , other : & Ident) -> Option < Ordering > { Some (self . cmp (other)) } }