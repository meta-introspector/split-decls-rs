use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl MockBinder { pub fn dummy < T > (_value : T) -> MockBinder { MockBinder } }
}