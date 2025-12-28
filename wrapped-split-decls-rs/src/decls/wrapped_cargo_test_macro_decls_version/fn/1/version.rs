use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn version () -> (u32 , bool) { LazyLock :: force (& VERSION) . clone () }
}