use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Default for AnalysisHost { fn default () -> AnalysisHost { AnalysisHost :: new (None) } }
}