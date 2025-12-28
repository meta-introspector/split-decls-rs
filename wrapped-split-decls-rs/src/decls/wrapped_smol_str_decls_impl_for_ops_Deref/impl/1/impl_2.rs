use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl ops :: Deref for SmolStr { type Target = str ; # [inline (always)] fn deref (& self) -> & str { self . as_str () } }