use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl hash::Hash for SmolStr {
    fn hash<H: hash::Hasher>(&self, hasher: &mut H) {
        self.as_str().hash(hasher);
    }
}
