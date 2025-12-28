use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type HammingResult = Result < usize , StrSimError > ;