use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < u128 > for Pu128 { # [inline] fn from (value : u128) -> Self { Self (value) } }