use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [doc = " Index of the matched macro arm on successful expansion."] pub type MatchedArmIndex = Option < u32 > ;
}