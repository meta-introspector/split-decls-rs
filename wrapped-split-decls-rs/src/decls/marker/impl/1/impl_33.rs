use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
unsafe impl < T : ? Sized + PointeeSized + Sync > DynSync for IntoDynSyncSend < T > { }
}