use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < N > IntoArrayLength for N where N : ArrayLength , { type ArrayLength = Self ; }
}