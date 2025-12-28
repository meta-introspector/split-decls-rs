use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl MockInferCtxtBuilder { pub fn build (self , _typing_mode : MockTypingMode) -> MockInferCtxt { MockInferCtxt } }
}