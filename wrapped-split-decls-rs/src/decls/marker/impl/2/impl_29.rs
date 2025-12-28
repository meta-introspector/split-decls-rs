use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > std :: ops :: Deref for FromDyn < T > { type Target = T ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } }