use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a > PartialEq < & 'a String > for SmolStr { # [inline (always)] fn eq (& self , other : & & 'a String) -> bool { self == * other } }