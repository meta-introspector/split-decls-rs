use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type LintStoreExpandDyn < 'a > = Option < & 'a (dyn LintStoreExpand + 'a) > ;