use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T : FiniteBitSetTy > Default for FiniteBitSet < T > { fn default () -> Self { Self :: new_empty () } }