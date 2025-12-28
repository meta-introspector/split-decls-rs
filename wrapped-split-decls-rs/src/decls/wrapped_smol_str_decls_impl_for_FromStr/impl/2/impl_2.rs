use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl FromStr for SmolStr { type Err = Infallible ; # [inline] fn from_str (s : & str) -> Result < SmolStr , Self :: Err > { Ok (SmolStr :: from (s)) } }