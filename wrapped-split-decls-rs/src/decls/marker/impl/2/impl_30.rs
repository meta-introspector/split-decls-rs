use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > std :: ops :: DerefMut for FromDyn < T > { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } }