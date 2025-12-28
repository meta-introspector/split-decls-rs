use serde::{Deserialize, Serialize};
use std::collections::HashMap;

unsafe impl < T : ? Sized + PointeeSized + Send > DynSend for IntoDynSyncSend < T > { }