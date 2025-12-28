use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < I : Idx , K : Eq , V : Eq > Eq for SortedIndexMultiMap < I , K , V > { }