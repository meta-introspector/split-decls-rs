use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Default for Span { fn default () -> Self { DUMMY_SP } }