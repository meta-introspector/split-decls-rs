use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl AsRef < str > for BaseNString { fn as_ref (& self) -> & str { self . buf [self . start ..] . as_str () } }