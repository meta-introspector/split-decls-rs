use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl PartialOrd for Local { fn partial_cmp (& self , other : & Self) -> Option < std :: cmp :: Ordering > { Some (self . cmp (other)) } }