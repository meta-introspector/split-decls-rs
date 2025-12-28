use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : Idx > Default for GrowableBitSet < T > { fn default () -> Self { GrowableBitSet :: new_empty () } }
}