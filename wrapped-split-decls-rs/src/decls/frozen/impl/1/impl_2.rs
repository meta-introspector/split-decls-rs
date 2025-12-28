use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > std :: ops :: Deref for Frozen < T > { type Target = T ; fn deref (& self) -> & T { & self . 0 } }
}