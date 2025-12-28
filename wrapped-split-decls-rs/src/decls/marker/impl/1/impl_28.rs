use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
unsafe impl < T : DynSync > Sync for FromDyn < T > { }
}