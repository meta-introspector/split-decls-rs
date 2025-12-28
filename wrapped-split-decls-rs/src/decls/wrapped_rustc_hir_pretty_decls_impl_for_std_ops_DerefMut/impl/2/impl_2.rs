use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl std :: ops :: DerefMut for State < '_ > { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . s } }
}