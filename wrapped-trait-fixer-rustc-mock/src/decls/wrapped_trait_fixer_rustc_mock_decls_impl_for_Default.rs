use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Default for MockTypingMode {
    fn default() -> Self {
        MockTypingMode::NonBodyAnalysis
    }
}
