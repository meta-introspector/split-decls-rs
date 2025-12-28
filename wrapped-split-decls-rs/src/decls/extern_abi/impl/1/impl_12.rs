use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl PartialOrd for ExternAbi { fn partial_cmp (& self , rhs : & Self) -> Option < Ordering > { Some (self . cmp (rhs)) } }