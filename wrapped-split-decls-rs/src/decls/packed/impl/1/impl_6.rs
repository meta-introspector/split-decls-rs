use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < Pu128 > for u128 { # [inline] fn from (value : Pu128) -> Self { value . get () } }