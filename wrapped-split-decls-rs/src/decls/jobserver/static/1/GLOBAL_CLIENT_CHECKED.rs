use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstatic! {
static GLOBAL_CLIENT_CHECKED : OnceLock < Client > = OnceLock :: new () ;
}