use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < G > ReversedGraph < G > { pub fn new (inner : G) -> Self { Self { inner } } }