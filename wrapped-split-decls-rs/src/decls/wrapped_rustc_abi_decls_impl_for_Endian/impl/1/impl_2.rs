use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Endian { pub fn as_str (& self) -> & 'static str { match self { Self :: Little => "little" , Self :: Big => "big" , } } }