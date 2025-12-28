use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : Copy + Debug + Eq + Hash + Ord > RuleType for T { }
}