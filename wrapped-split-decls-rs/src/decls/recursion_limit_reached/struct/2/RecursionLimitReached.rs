use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct RecursionLimitReached { pub span : Span , pub descr : String , pub suggested_limit : Limit , pub crate_name : Symbol , }
}