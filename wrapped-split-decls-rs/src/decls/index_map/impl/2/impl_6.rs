use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < I : Idx , K : PartialEq , V : PartialEq > PartialEq for SortedIndexMultiMap < I , K , V > { fn eq (& self , other : & Self) -> bool { self . items == other . items } }