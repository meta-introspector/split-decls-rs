use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
# [cfg (feature = "utf8")] impl utf8 :: Receiver for VtUtf8Receiver < '_ > { fn codepoint (& mut self , c : char) { * self . 0 = Some (c) ; } fn invalid_sequence (& mut self) { * self . 0 = Some ('�') ; } }
}