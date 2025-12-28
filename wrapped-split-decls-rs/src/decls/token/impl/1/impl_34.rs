use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl PartialEq < TokenKind > for Token { # [inline] fn eq (& self , rhs : & TokenKind) -> bool { self . kind == * rhs } }