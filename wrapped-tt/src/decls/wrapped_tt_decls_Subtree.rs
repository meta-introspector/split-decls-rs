use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Subtree<S> {
    pub delimiter: Delimiter<S>,
    /// Number of following token trees that belong to this subtree, excluding this subtree.
    pub len: u32,
}
