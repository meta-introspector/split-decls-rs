use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl core :: str :: FromStr for Hash { type Err = HexError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Hash :: from_hex (s) } }
}