use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < I : Idx , K , V > Hash for SortedIndexMultiMap < I , K , V > where K : Hash , V : Hash , { fn hash < H : Hasher > (& self , hasher : & mut H) { self . items . hash (hasher) } }