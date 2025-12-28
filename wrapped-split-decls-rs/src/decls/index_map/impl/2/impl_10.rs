use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < I : Idx , K , V > std :: ops :: Index < I > for SortedIndexMultiMap < I , K , V > { type Output = V ; fn index (& self , idx : I) -> & Self :: Output { & self . items [idx] . 1 } }