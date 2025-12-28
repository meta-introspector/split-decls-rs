use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < S > Deref for UniCase < S > { type Target = S ; # [inline] fn deref < 'a > (& 'a self) -> & 'a S { inner ! (self . 0) } }