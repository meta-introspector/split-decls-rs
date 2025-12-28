use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl AsRef < str > for SmolStr { # [inline (always)] fn as_ref (& self) -> & str { self . as_str () } }