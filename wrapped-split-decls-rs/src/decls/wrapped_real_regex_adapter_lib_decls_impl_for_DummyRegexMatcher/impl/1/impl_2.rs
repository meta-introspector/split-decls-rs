use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl DummyRegexMatcher { pub fn new (_re : & str) -> Result < Self > { Ok (DummyRegexMatcher) } }
}