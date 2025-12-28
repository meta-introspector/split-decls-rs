use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl ArgCursor { fn new () -> Self { Self { cursor : 0 } } }
}