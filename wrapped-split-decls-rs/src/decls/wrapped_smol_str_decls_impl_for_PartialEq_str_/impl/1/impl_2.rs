use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl PartialEq < str > for SmolStr { # [inline (always)] fn eq (& self , other : & str) -> bool { self . as_str () == other } }