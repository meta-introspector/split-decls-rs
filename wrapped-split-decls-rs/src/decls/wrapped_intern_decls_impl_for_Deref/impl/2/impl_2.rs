use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T : Internable + ? Sized > Deref for Interned < T > { type Target = T ; # [inline] fn deref (& self) -> & Self :: Target { & self . arc } }