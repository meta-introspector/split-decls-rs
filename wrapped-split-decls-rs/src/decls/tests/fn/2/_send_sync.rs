use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Helper used by check_error_send_sync to ensure ThreadPoolBuildError is Send + Sync"] fn _send_sync < T : Send + Sync > () { }