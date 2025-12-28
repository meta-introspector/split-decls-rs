use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl DelimArgs { # [doc = " Whether a macro with these arguments needs a semicolon"] # [doc = " when used as a standalone item or statement."] pub fn need_semicolon (& self) -> bool { ! matches ! (self , DelimArgs { delim : Delimiter :: Brace , .. }) } }