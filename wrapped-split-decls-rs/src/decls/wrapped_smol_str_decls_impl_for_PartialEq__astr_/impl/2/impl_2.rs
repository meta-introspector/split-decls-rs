use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a > PartialEq < & 'a str > for SmolStr { # [inline (always)] fn eq (& self , other : & & 'a str) -> bool { self == * other } }