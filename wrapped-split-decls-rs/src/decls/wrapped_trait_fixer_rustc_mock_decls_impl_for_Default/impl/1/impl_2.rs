use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Default for MockTypingMode { fn default () -> Self { MockTypingMode :: NonBodyAnalysis } }
}