use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn scope_empty () { scope (| _ | { }) ; }
}