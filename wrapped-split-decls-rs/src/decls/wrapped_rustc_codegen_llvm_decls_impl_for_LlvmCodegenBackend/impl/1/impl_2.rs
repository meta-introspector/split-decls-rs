use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl LlvmCodegenBackend { pub fn new () -> Box < dyn CodegenBackend > { Box :: new (LlvmCodegenBackend (())) } }
}