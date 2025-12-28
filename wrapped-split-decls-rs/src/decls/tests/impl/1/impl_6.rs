use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl StartNode for TestGraph { fn start_node (& self) -> usize { self . start_node } }
}