use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'a> fmt::Debug for TrieSetSlice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TrieSetSlice(...)")
    }
}
