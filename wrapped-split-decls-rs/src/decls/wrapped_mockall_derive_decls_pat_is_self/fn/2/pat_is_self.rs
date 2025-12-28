use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Determine if this Pat is any kind of `self` binding"] fn pat_is_self (pat : & Pat) -> bool { if let Pat :: Ident (pi) = pat { pi . ident == "self" } else { false } }