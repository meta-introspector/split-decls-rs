use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Default for DetectionState { fn default () -> Self { Builder :: default () . into_detection_state () . expect ("enabled by default") } }
}