use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > std :: ops :: Deref for IntoDynSyncSend < T > { type Target = T ; # [inline (always)] fn deref (& self) -> & T { & self . 0 } }
}