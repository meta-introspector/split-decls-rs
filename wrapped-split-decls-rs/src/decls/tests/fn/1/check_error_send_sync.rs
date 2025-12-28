use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn check_error_send_sync () { _send_sync :: < ThreadPoolBuildError > () ; }
}