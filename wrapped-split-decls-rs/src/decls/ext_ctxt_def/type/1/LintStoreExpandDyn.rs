use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type LintStoreExpandDyn < 'a > = Option < & 'a (dyn LintStoreExpand + 'a) > ;
}