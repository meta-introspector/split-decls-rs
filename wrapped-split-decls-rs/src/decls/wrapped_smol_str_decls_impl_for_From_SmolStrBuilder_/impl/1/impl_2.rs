use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < SmolStrBuilder > for SmolStr { fn from (value : SmolStrBuilder) -> Self { value . finish () } }