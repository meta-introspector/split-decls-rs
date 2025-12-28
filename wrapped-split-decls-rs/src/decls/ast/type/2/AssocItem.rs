use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Represents associated items."] # [doc = " These include items in `impl` and `trait` definitions."] pub type AssocItem = Item < AssocItemKind > ;