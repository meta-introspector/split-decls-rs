use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl FunctionVisitor { fn new () -> Self { Self { call_count : 0 } } }
}