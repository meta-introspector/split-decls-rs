use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [cfg (not (feature = "send_guard"))] type GuardMarker = lock_api :: GuardNoSend ;
}