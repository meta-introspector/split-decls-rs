use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Marker type which indicates that the Guard type for a lock is not `Send`."] # [allow (dead_code)] pub struct GuardNoSend (* mut ()) ;
}