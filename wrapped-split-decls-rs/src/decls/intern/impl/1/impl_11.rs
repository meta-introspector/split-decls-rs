use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a , T > Deref for Interned < 'a , T > { type Target = T ; # [inline] fn deref (& self) -> & T { self . 0 } }