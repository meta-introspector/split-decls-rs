use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T : Idx > Default for GrowableBitSet < T > { fn default () -> Self { GrowableBitSet :: new_empty () } }