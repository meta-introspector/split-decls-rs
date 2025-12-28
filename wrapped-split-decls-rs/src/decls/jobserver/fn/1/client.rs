use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn client () -> Client { GLOBAL_CLIENT_CHECKED . get () . expect (ACCESS_ERROR) . clone () }
}