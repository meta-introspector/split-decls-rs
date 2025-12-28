use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
unsafe impl < T : ? Sized + PointeeSized + Send > DynSend for IntoDynSyncSend < T > { }
}