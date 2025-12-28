use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < Group > for TokenTree { fn from (g : Group) -> Self { TokenTree :: Group (g) } }