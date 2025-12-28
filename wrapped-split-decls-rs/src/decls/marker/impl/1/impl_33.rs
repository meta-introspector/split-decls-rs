use serde::{Deserialize, Serialize};
use std::collections::HashMap;

unsafe impl < T : ? Sized + PointeeSized + Sync > DynSync for IntoDynSyncSend < T > { }