use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > Default for ThinVec < T > { fn default () -> ThinVec < T > { ThinVec :: new () } }