use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < V : Eq + Hash > UnordCollection for UnordSet < V > { }