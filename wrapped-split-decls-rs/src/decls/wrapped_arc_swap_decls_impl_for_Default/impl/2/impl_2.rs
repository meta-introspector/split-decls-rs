use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T : RefCnt + Default , S : Default + Strategy < T > > Default for ArcSwapAny < T , S > { fn default () -> Self { Self :: new (T :: default ()) } }