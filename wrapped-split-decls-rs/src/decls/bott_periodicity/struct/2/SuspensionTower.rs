use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Suspending 8 times returns you to where you started (up to isomorphism)"] pub struct SuspensionTower { levels : Vec < AbstractionBundle > , }
}