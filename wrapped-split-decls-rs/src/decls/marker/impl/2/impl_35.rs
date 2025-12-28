use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > std :: ops :: DerefMut for IntoDynSyncSend < T > { # [inline (always)] fn deref_mut (& mut self) -> & mut T { & mut self . 0 } }