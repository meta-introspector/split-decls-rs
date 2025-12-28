use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < G > ReversedGraph < G > { pub fn new (inner : G) -> Self { Self { inner } } }
}