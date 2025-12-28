use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < K : Eq + Hash , V > UnordCollection for UnordMap < K , V > { }