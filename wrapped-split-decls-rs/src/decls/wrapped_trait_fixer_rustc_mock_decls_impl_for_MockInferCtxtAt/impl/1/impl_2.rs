use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl MockInferCtxtAt { pub fn predicate_may_hold (self , _predicate : & MockPredicate) -> bool { true } }
}