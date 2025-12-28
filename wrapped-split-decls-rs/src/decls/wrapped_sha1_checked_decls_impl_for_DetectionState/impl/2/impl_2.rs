use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl DetectionState { fn reset (& mut self) { self . found_collision = false ; self . ihv1 = Default :: default () ; self . ihv2 = Default :: default () ; self . m1 = [0 ; 80] ; self . m2 = [0 ; 80] ; self . state_58 = Default :: default () ; self . state_65 = Default :: default () ; } }
}