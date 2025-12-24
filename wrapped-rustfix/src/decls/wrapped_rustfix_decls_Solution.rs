use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Solution to a diagnostic item.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Solution {
    /// The error message of the diagnostic item.
    pub message: String,
    /// Possible solutions to fix the error.
    pub replacements: Vec<Replacement>,
}
