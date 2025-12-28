use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Fields of a braced struct syntax tree node with named fields."] # [doc = ""] # [doc = " The keys in the map are the field names."] pub type Fields = IndexMap < String , Type > ;