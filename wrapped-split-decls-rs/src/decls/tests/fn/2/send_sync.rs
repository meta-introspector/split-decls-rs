use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn send_sync () { crate :: sync :: assert_dyn_send :: < OwnedSlice > () ; crate :: sync :: assert_dyn_sync :: < OwnedSlice > () ; }